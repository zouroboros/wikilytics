use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use quick_xml::reader::Reader;
use quick_xml::events::Event;

use itertools::Itertools;

pub struct WikiXmlDump<R> {
    reader: Reader<R>
}

#[derive(Debug)]
pub struct WikiPage {
    pub title: String,
    pub text: Option<String>,
    pub namespace_id: i16
}

impl<R: BufRead> WikiXmlDump<R> {
    pub fn new(reader: Reader<R>) -> WikiXmlDump<R> {
        WikiXmlDump {
            reader: reader,
        }
    }

    pub fn read_base(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        let mut found_base = false;

        loop {
            match self.reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", self.reader.buffer_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => found_base = e.name().as_ref() == b"base",
                Ok(Event::Text(e)) => {
                    if found_base {
                        return Some(e.unescape().unwrap().into_owned())
                    }
                }
                _ => (),
            }
        }

        None
    }
}

fn read_page<R: BufRead>(reader: &mut Reader<R>) -> Option<WikiPage> {
    let mut buf = Vec::new();

    let mut is_title = false;
    let mut is_text = false;
    let mut is_namespace = false;

    let mut title_option: Option<String> = None;
    let mut text_option: Option<String> = None;
    let mut namespace_id_option: Option<i16> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => return None,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        is_title = true;
                    }
                    b"text" => {
                        is_text = true;
                    }
                    b"ns" => {
                        is_namespace = true;
                    }
                    _ => ()
                }
            }
            Ok(Event::Text(e)) => {
                if is_title {
                    title_option = Some(e.unescape().unwrap().into_owned());
                }

                if is_text {
                    text_option = Some(e.unescape().unwrap().into_owned());
                }
                
                if is_namespace {
                    namespace_id_option = e.unescape().unwrap().parse::<i16>().ok();
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        is_title = false;
                    }
                    b"text" => {
                        is_text = false;
                    }
                    b"ns" => {
                        is_namespace = false;
                    }
                    b"page" => {
                        break;
                    }
                    _ => ()
                }
            }
            _ => (),
        }
    }

    Some(WikiPage{ 
        title: title_option.expect("A page must have a title!"), 
        text: text_option, 
        namespace_id: namespace_id_option.expect("A page must have a namespace!") 
    })
}

impl<R: BufRead> Iterator for WikiXmlDump<R> {
    type Item = WikiPage;

    fn next(&mut self) -> Option<Self::Item> {
        read_page(&mut self.reader)
    }
}

pub type WikiIndex = Vec<WikiIndexEntry>;

#[derive(Debug)]
pub struct WikiIndexEntry {
    pub start: u64,
    pub id: u64,
    pub title: String
}

pub fn read_index<R: Read>(reader: BufReader<R>) -> Result<WikiIndex, Error> {
    let read_line = |line: String| {
        let mut parts = line.split(":");
        let start = parts.next()
            .ok_or(Error::from(ErrorKind::Other))?
            .parse::<u64>()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let id = parts.next()
            .ok_or(Error::from(ErrorKind::Other))?
            .parse::<u64>()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        let title = parts.next()
            .ok_or(Error::from(ErrorKind::Other))?
            .to_owned();

        Ok(WikiIndexEntry{ start, id, title })
    };

    reader.lines().map(|line| { read_line(line?) }).collect()
}

pub fn blocks(index: WikiIndex) -> Vec<u64> {
    index.iter().group_by(|entry| entry.start)
        .into_iter()
        .map(|(key, _)| key)
        .collect()
}

