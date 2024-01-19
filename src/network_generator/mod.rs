use std::{collections::HashMap, io::BufRead};

use self::{wiki_pages::WikiPages, wiki_text::linked_articles};

pub mod wiki_pages;
pub mod wiki_text;

pub fn generate_network<T: BufRead>(pages: WikiPages<T>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();

    for page in pages {
        adjacency.insert(page.title.to_owned(), linked_articles(&page));
    }

    adjacency
}