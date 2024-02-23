use std::{cmp::min, collections::HashMap, fs::File, io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write}, path::PathBuf, sync::mpsc::{channel, Sender}, thread};
use std::io::Result;

use bzip2::{bufread::MultiBzDecoder, read};
use itertools::Itertools;
use quick_xml::Reader;

use crate::network_generator::{generate_network, generate_network_parrallel, wiki_xml_dump::{blocks, read_index, WikiXmlDump}};

pub fn network(xml_dump_path: PathBuf, dump_index_path: PathBuf, network_file_path: PathBuf) -> Result<()> {
    let number_of_threads = 6;
    
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

    let (sender, receiver) = channel();

    for thread_number in 0..number_of_threads {
        let block_start = blocks[thread_number];
        let block_end = blocks[thread_number + 1];
        let xml_dump_path = xml_dump_path.clone();
        let sender = sender.clone();

        thread::spawn(move || {
           process_partial_dump(xml_dump_path, block_start, block_end, sender).unwrap();
        });
    }

    drop(sender);

    let number_of_pages = receiver.into_iter().fold(0, |acc, count| acc + count);

    println!("{number_of_pages}");

    /*let file = File::open(xml_dump_path)?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);

    let bz_reader = BufReader::new(bz_decoder);

    let reader = Reader::from_reader(bz_reader);

    let xml_dump = WikiXmlDump::new(reader);

    //let base = xml_dump.read_base().unwrap();

    let network = generate_network_parrallel(xml_dump, 1000, 4);

    save_network(network, network_file_path)?;

    /*let statistics = gather_statistics(&network, base);

    let statistics_file = File::create("../viewer/public/statistics.json")?;
    let statistics_writer = BufWriter::new(statistics_file);

    serde_json::to_writer(statistics_writer, &statistics)?;*/* */

    Ok(())
}

fn process_partial_dump(xml_dump_path: PathBuf, block_start: u64, block_end: u64, sender: Sender<u64>) -> Result<()> {
    let mut file = File::open(xml_dump_path)?;
    file.seek(SeekFrom::Start(block_start))?;

    let number_of_bytes = block_end - block_start;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader.take(number_of_bytes));
    let bz_reader = BufReader::new(bz_decoder);
    let mut reader = Reader::from_reader(bz_reader);
    reader.check_end_names(false);
    let xml_dump = WikiXmlDump::new(reader);


    sender.send(xml_dump.into_iter().fold(0, |acc, _| acc + 1)).unwrap();

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

fn save_network(network_to_save: HashMap<String, Vec<String>>, save_file_path: PathBuf) -> Result<()> {
    let save_file = File::create(save_file_path)?;
    let mut file_writer = BufWriter::new(save_file);

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