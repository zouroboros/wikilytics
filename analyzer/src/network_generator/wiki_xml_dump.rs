use std::io::BufRead;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

pub struct WikiXmlDump<R> {
    reader: Reader<R>
}

pub struct WikiPage {
    pub title: String,
    pub text: String,
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
            Ok(Event::Eof) => break,
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

    Some(WikiPage{ title: title_option?, text: text_option?, namespace_id: namespace_id_option? })
}

impl<R: BufRead> Iterator for WikiXmlDump<R> {
    type Item = WikiPage;

    fn next(&mut self) -> Option<Self::Item> {
        read_page(&mut self.reader)
    }
}

