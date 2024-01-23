use std::{collections::HashMap, io::BufRead};

use self::{wiki_xml_dump::WikiXmlDump, wiki_text::linked_articles, wiki_text::is_redirect, wiki_text::parse_text};

pub mod wiki_xml_dump;
pub mod wiki_text;

pub fn generate_network<T: BufRead>(pages: WikiXmlDump<T>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();

    for page in pages {
        let text = parse_text(&page);

        if page.namespace_id == 0 && !is_redirect(&text) {
            adjacency.insert(page.title.to_owned(), linked_articles(text));
        }

    }

    remove_dangling_links(adjacency)
}

fn remove_dangling_links(network: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    // TODO zouroboros 2024-01-23 there must a way to do this more efficiently
    HashMap::from_iter(network.iter()
        .map(|(page, links)| 
            (page.to_owned(), links.iter()
                .filter(|link| network.contains_key(*link))
                .map(|link| link.to_owned())
                .collect::<Vec<String>>())
            ))
}