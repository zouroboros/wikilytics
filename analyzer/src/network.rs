use std::{cmp::min, collections::{HashMap, HashSet}, fs::{remove_file, rename, File}, io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Read, Seek, SeekFrom, Write}, path::PathBuf, sync::mpsc::{channel, Sender}, thread};
use std::io::Result;

use bzip2::bufread::MultiBzDecoder;
use itertools::Itertools;
use quick_xml::Reader;

use crate::network_generator::{canonicalize_link, generate_network, generate_network_parrallel, wiki_text::{is_redirect, linked_articles, parse_text, redirects_to}, wiki_xml_dump::{blocks, read_index, WikiPage, WikiXmlDump}};

pub fn network(xml_dump_path: PathBuf, dump_index_path: PathBuf, network_file_path: PathBuf) -> Result<()> {
    let number_of_threads = 4;
    
    let start_positions = read_index_file(dump_index_path)?;
    let block_size = start_positions.len() / number_of_threads + 1;
    let mut blocks = Vec::with_capacity(number_of_threads + 1);
    blocks.push(0);

    for thread_number in 1..number_of_threads {
        let start_index = thread_number * block_size;
        blocks.push(start_positions[start_index]);
    }

    let file_size = File::open(xml_dump_path.clone())?.metadata()?.len();
    blocks.push(file_size + 1);

    println!("{:?}", blocks);

    let (adjacency_sender, adjacency_receiver) = channel();
    let (redirect_sender, redirect_receiver) = channel();

    for thread_number in 0..number_of_threads {
        let block_start = blocks[thread_number];
        let block_end = blocks[thread_number + 1];
        let xml_dump_path = xml_dump_path.clone();
        let adjacency_sender = adjacency_sender.clone();
        let redirect_sender = redirect_sender.clone();

        thread::spawn(move || {
            println!("starting decoding thread {thread_number}");
            process_partial_dump(xml_dump_path, block_start, block_end, adjacency_sender, redirect_sender).unwrap();
            println!("finished decoding thread {thread_number}");
        });
    }

    drop(redirect_sender);
    drop(adjacency_sender);

    let (finish_sender, finish_receiver) = channel();

    {
        let finish_sender = finish_sender.clone();
        thread::spawn(move || {
            println!("starting to save network");
            save_network(adjacency_receiver.into_iter(), network_file_path).unwrap();
            println!("finished saving the network");
            finish_sender.send(()).unwrap();
        });
    }

    thread::spawn(move || {
        println!("starting to process redirects");
        save_redirects(redirect_receiver.into_iter(), PathBuf::from("redirects.csv")).unwrap();
        println!("finished saving redirects");
        close_redirects(PathBuf::from("redirects.csv")).unwrap();
        println!("finished closing redirects");

        finish_sender.send(()).unwrap();
    });

    finish_receiver.recv().unwrap();
    finish_receiver.recv().unwrap();

    Ok(())
}

fn process_partial_dump(xml_dump_path: PathBuf, block_start: u64, block_end: u64, adjacency_sender: Sender<(String, Vec<String>)>, redirect_sender: Sender<(String, String)>) -> Result<()> {
    let mut file = File::open(xml_dump_path)?;
    file.seek(SeekFrom::Start(block_start))?;

    let number_of_bytes = block_end - block_start;
    let file_reader = BufReader::with_capacity(16 * 1024 * 1024, file.take(number_of_bytes));
    let bz_decoder = MultiBzDecoder::new(file_reader);
    let bz_reader = BufReader::new(bz_decoder);
    let mut reader = Reader::from_reader(bz_reader);
    reader.check_end_names(false);
    let xml_dump = WikiXmlDump::new(reader);

    for page in xml_dump {
        if page.namespace_id == 0 {
            let links = parse_text(&page);
    
            if let Some(links) = links {
                
                if !is_redirect(&links) {
                    let links = linked_articles(&links).iter()
                        .filter_map(|link| canonicalize_link(*link))
                        .collect();
                    adjacency_sender.send((page.title, links)).unwrap();
                } else {

                    if let Some(target) = redirects_to(&links).and_then(canonicalize_link) {

                        redirect_sender.send((page.title, target)).unwrap();
                    } 
                }
            }
        }
    }

    Ok(())
}

fn read_index_file(dump_index_path: PathBuf) -> Result<Vec<u64>> {
    let file = File::open(dump_index_path)?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);

    let bz_reader = BufReader::new(bz_decoder);

    let index = read_index(bz_reader)?;

    Ok(blocks(index))
}

fn save_network<T>(network_to_save: T, save_file_path: PathBuf) -> Result<()> where T: Iterator<Item = (String, Vec<String>)> {
    let save_file = File::create(save_file_path)?;
    let mut file_writer = BufWriter::with_capacity(128 * 1024 * 1024, save_file);

    let separator = "; ".as_bytes();
    let newline = "\n".as_bytes();

    for (node, connected_nodes) in network_to_save {
        file_writer.write(node.as_bytes())?;

        for connected_node in connected_nodes {
            file_writer.write(separator)?;
            file_writer.write(connected_node.as_bytes())?;
        }

        file_writer.write(newline)?;
    }

    Ok(())
}

fn save_redirects<T>(network_to_save: T, save_file_path: PathBuf) -> Result<()> where T: Iterator<Item = (String, String)> {
    let save_file = File::create(save_file_path)?;
    let mut file_writer = BufWriter::with_capacity(16 * 1024 * 1024,save_file);

    let separator = ";".as_bytes();
    let newline = "\n".as_bytes();

    for (node, target) in network_to_save {
        file_writer.write(node.as_bytes())?;
        file_writer.write(separator)?;
        file_writer.write(target.as_bytes())?;
        file_writer.write(newline)?;
    }

    Ok(())
}

fn close_redirects(mut redirect_file: PathBuf) -> Result<()> {
    let batch_size = 10000;
    let original_redirect_file_path = redirect_file.clone();
    redirect_file.set_extension("temp");
    let temporary_redirect_file_path = redirect_file.clone();

    let original_redirect_file = File::open(original_redirect_file_path.clone())?;
    let reader = BufReader::new(original_redirect_file);
    let mut writer = BufWriter::new(File::create(temporary_redirect_file_path.clone())?);

    let mut lines = reader.lines().peekable();

    let mut batch: Vec<(String, String)> = Vec::with_capacity(batch_size);
    let mut cycles = HashMap::new();

    while lines.peek().is_some() || batch.len() > 0 {
        let new_batch = lines.by_ref()
            .take(batch_size - batch.len())
            .map(|line| parse_redirects(line?))
            .collect::<Result<Vec<(String, String)>>>()?;

        batch.extend(new_batch);

        let targets = batch.iter()
            .map(|(_, target)| target)
            .collect::<HashSet<&String>>();

        let resolved_targets = resolve_redirects(original_redirect_file_path.clone(), targets)?;

        let mut unresolved_redirects = Vec::new();

        batch.retain_mut(|tuple| {
            let (link, target) = tuple;

            if let Some(new_target) = resolved_targets.get(target) {
                if target != new_target {
                    
                    if !cycles.contains_key(link) {
                        cycles.insert(link.to_owned(), Vec::new());
                    } 
                    
                    if let Some(path) = cycles.get_mut(link) {
                        if path.contains(new_target) {
                            println!("found cyclic redirects for {} resolved path {:?}", link, path);
                            return false;
                        }

                        path.push(new_target.to_owned());
                    }

                    tuple.1 = new_target.to_owned();
                    return true;
                }
            } 
            
            cycles.remove(link);
            unresolved_redirects.push((link.to_owned(), target.to_owned()));
            false
        });

        for (link, target) in unresolved_redirects {
            writer.write_all(link.as_bytes())?;
            writer.write_all(b";")?;
            writer.write_all(target.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        
    }

    //remove_file(original_redirect_file_path.clone())?;
    //rename(temporary_redirect_file_path, original_redirect_file_path)?;

    Ok(())

}

fn parse_redirects(line: String) -> Result<(String, String)> {
    let mut parts = line.split(";");

    if let Some((link, target)) = parts.next_tuple() {
        return Ok((link.to_owned(), target.to_owned()));
    }

    println!("error in line '{:?}'", line);
    Err(Error::from(ErrorKind::InvalidInput))
}

fn resolve_redirects(redirect_file_path: PathBuf, links: HashSet<&String>) -> Result<HashMap<String, String>> {
    let reader = BufReader::new(File::open(redirect_file_path)?);
    let mut results = HashMap::new();

    for line in reader.lines() {
        let (original_target, new_target) = parse_redirects(line?)?;

        if links.contains(&original_target) {
            results.insert(original_target, new_target.to_owned());
        }

        if results.len() == links.len() {
            return Ok(results);
        }
    }

    return Ok(results);
}