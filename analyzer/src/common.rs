use std::{fs::File, io::{BufRead, BufReader, Read, Result, Seek, SeekFrom}, path::PathBuf};

use bzip2::bufread::MultiBzDecoder;
use quick_xml::Reader;

use crate::network_generator::wiki_xml_dump::{read_index, WikiIndexEntry, WikiXmlDump};

pub fn read_from(xml_dump_path: &PathBuf, block_start: u64) -> Result<WikiXmlDump<impl BufRead>> {
    let mut file = File::open(xml_dump_path)?;
    file.seek(SeekFrom::Start(block_start))?;

    let buf_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(buf_reader);
    let bz_reader = BufReader::new(bz_decoder);
    let mut reader = Reader::from_reader(bz_reader);
    reader.check_end_names(false);

    let xml_dump = WikiXmlDump::new(reader); 

    Ok(xml_dump)
}

pub fn read_from_to(xml_dump_path: &PathBuf, block_start: u64, block_end: u64) -> Result<WikiXmlDump<impl BufRead>> {
    let mut file = File::open(xml_dump_path)?;
    file.seek(SeekFrom::Start(block_start))?;

    let buf_reader = BufReader::new(file.take(block_end - block_start));
    let bz_decoder = MultiBzDecoder::new(buf_reader);
    let bz_reader = BufReader::new(bz_decoder);
    let mut reader = Reader::from_reader(bz_reader);
    reader.check_end_names(false);

    let xml_dump = WikiXmlDump::new(reader); 

    Ok(xml_dump)
}

pub fn find_entries(xml_dump_index_path: &PathBuf, title: &String) -> Result<Vec<WikiIndexEntry>> {
    let file = File::open(xml_dump_index_path)?;
    let file_reader = BufReader::new(file);
    let bz_decoder = MultiBzDecoder::new(file_reader);

    let bz_reader = BufReader::new(bz_decoder);

    let wiki_index = read_index(bz_reader)?;

    let entries = wiki_index.into_iter()
        .filter(|page| page.title == *title)
        .collect();

    Ok(entries)
}