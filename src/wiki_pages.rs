use std::io::BufRead;
use quick_xml::reader::Reader;
use quick_xml::events::{Event};

pub struct WikiPages<R> {
    reader: Reader<R>
}

pub struct WikiPage {
    pub title: String,
    pub text: String
}

impl<R: BufRead> WikiPages<R> {
    pub fn new(reader: Reader<R>) -> WikiPages<R> {
        WikiPages {
            reader: reader,
        }
    }
}

fn read_page<R: BufRead>(reader: &mut Reader<R>) -> Option<WikiPage> {
    let mut buf = Vec::new();

    let mut is_title = false;
    let mut is_text = false;

    let mut title_option: Option<String> = None;
    let mut text_option: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        is_title = true;
                    }
                    b"text" => {
                        is_text = true;
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
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"title" => {
                        is_title = false;
                    }
                    b"text" => {
                        is_text = false;
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

    Some(WikiPage{ title: title_option?, text: text_option? })
}

impl<R: BufRead> Iterator for WikiPages<R> {
    type Item = WikiPage;

    fn next(&mut self) -> Option<Self::Item> {
        read_page(&mut self.reader)
    }
}

