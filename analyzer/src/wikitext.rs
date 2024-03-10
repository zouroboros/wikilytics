use std::{fs::File, io::BufReader, io::Result, path::PathBuf};

use bzip2::bufread::MultiBzDecoder;

use crate::{common::read_from, network_generator::wiki_xml_dump::{read_index, WikiIndex, WikiIndexEntry}};

pub fn wikitext(xml_dump_path: &PathBuf, xml_dump_index_file_path: &PathBuf, title: &String) -> Result<()> {
    let file = File::open(xml_dump_index_file_path)?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);

    let bz_reader = BufReader::new(bz_decoder);

    let wiki_index = read_index(bz_reader)?;

    if let Some(entry) = find_entry(&wiki_index, title) {
        let pages = read_from(xml_dump_path, entry.start)?;
        for page in pages {
            if page.title == entry.title {
                let text = page.text.unwrap_or("No wikitext!".to_owned());
                println!("{}", text);
                return Ok(());
            }
        }
    }

    println!("No article with title {title} found!");

    Ok(())
}

fn find_entry<'a>(index: &'a WikiIndex, title: &String) -> Option<&'a WikiIndexEntry> {
    for entry in index {
        if entry.title == *title {
            return Some(&entry)
        }
    }

    None
}