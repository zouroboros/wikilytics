use std::{collections::HashMap, io::BufRead};

use self::{wiki_pages::WikiPages, wiki_text::linked_articles, wiki_text::is_redirect, wiki_text::parse_text};

pub mod wiki_pages;
pub mod wiki_text;

pub fn generate_network<T: BufRead>(pages: WikiPages<T>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();

    for page in pages {
        let text = parse_text(&page);

        if !is_redirect(&text) {
            adjacency.insert(page.title.to_owned(), linked_articles(text));
        }

    }

    adjacency
}