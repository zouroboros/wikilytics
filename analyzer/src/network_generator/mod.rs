use std::{collections::HashMap, io::BufRead};

use self::{wiki_xml_dump::WikiXmlDump, wiki_text::linked_articles, wiki_text::is_redirect, wiki_text::redirects_to, wiki_text::parse_text};

pub mod wiki_xml_dump;
pub mod wiki_text;

pub fn generate_network<T: BufRead>(pages: WikiXmlDump<T>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();
    let mut redirects = HashMap::new();

    for page in pages {

        println!("{:?}", page.title);

        if page.namespace_id == 0 {
            println!("start parseing");
            let links = parse_text(&page);
            println!("finished parseing");


            if let Some(links) = links {
                
                if !is_redirect(&links) {
                
                    let links = linked_articles(&links).iter()
                        .filter_map(|link| canonicalize_link(*link))
                        .collect();
                    adjacency.insert(page.title, links);
        
                } else {
                    if let Some(target) = redirects_to(&links).and_then(canonicalize_link) {

                        redirects.insert(page.title, target);
                    }           
                }
            }
        }

        
    }

    println!("scanning done");

    let redirects = close_redirects(redirects);

    println!("close redirects done");

    resolve_redirects(&mut adjacency, redirects);

    println!("resolve redirects done");


    remove_dangling_links(adjacency)

}

fn canonicalize_link(link: &String) -> Option<String> {
    let mut chars = link.chars();
    let first_letter = chars.next()?;
    let rest = &chars.collect::<String>();

    Some(first_letter.to_ascii_uppercase().to_string() + rest)
}


fn close_redirects(redirects: HashMap<String, String>) -> HashMap<String, String> {
    let mut resolved_redirects = HashMap::with_capacity(redirects.len());
    
    for (link, mut target) in &redirects {

        while let Some(new_target) = redirects.get(target) {
            
            if new_target == target {
                break;
            }
            
            target = new_target;
        }
        
        resolved_redirects.insert(link.to_owned(), target.to_owned());
    }

    resolved_redirects
}


fn resolve_redirects(network: &mut HashMap<String, Vec<String>>, redirects: HashMap<String, String>) {
    for (_, links) in network.iter_mut() {
        for link in links.iter_mut() {
            if let Some(target) = redirects.get(link) {
                link.clear();
                link.push_str(&target);
            }
        }
    }
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