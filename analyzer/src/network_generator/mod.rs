use std::{collections::HashMap, io::BufRead, sync::mpsc::{channel, sync_channel, SyncSender, TrySendError}, thread};

use self::{wiki_text::{is_redirect, linked_articles, parse_text, redirects_to}, wiki_xml_dump::{WikiPage, WikiXmlDump}};

pub mod wiki_xml_dump;
pub mod wiki_text;

pub fn generate_network<T: BufRead>(pages: WikiXmlDump<T>) -> HashMap<String, Vec<String>> {
    let mut adjacency = HashMap::new();
    let mut redirects = HashMap::new();

    for page in pages {
        process_page(&mut adjacency, &mut redirects, page);
    }

   remove_redirects(adjacency, redirects)
}

pub fn generate_network_parrallel<T: BufRead>(pages: WikiXmlDump<T>, max_queue_size: usize, number_of_threads: usize) -> HashMap<String, Vec<String>> {
    let mut senders: Vec<SyncSender<WikiPage>> = Vec::with_capacity(number_of_threads);
    let (result_sender, result_receiver) = channel();

    for _ in 0..number_of_threads {
        let (sender, receiver) = sync_channel(max_queue_size);
        senders.push(sender);
        
        let result_sender = result_sender.clone();

        thread::spawn(move || {
            let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
            let mut redirects = HashMap::new();

            for page in receiver {
                process_page(&mut adjacency, &mut redirects, page);
            }

            result_sender.send((adjacency, redirects)).unwrap();
        });
    }

    let mut channel_index = 0;
    for mut page in pages {
        let mut sender = &senders[channel_index];

        while let Err(TrySendError::Full(new_page)) = sender.try_send(page) {
            channel_index = (channel_index + 1) % number_of_threads;
            sender = &senders[channel_index];
            page = new_page
        }

        channel_index = (channel_index + 1) % number_of_threads;
    }

    for sender in senders {
        drop(sender);
    }

    let mut whole_network = HashMap::new();
    let mut all_redirects = HashMap::new();

    for _ in 0..number_of_threads {
        let (partial_network, some_redirects) = result_receiver.recv().unwrap();

        whole_network.reserve(partial_network.len());
        all_redirects.reserve(some_redirects.len());
        
        for (node, links) in partial_network {
            whole_network.insert(node, links);
        }

        for (node, link) in some_redirects {
            all_redirects.insert(node, link);
        }
    }

    drop(result_sender);

   remove_redirects(whole_network, all_redirects)
}

fn process_page(network: &mut HashMap<String, Vec<String>>, redirects: &mut HashMap<String, String>, page: WikiPage) {
    if page.namespace_id == 0 {
        let links = parse_text(&page);

        if let Some(links) = links {
            
            if !is_redirect(&links) {
            
                let links = linked_articles(&links).iter()
                    .filter_map(|link| canonicalize_link(*link))
                    .collect();
                network.insert(page.title, links);
    
            } else {
                if let Some(target) = redirects_to(&links).and_then(canonicalize_link) {

                    redirects.insert(page.title, target);
                }           
            }
        }
    }
}

fn remove_redirects(mut network: HashMap<String, Vec<String>>, redirects: HashMap<String, String>) -> HashMap<String, Vec<String>> {
    let all_redirects = close_redirects(redirects);
    resolve_redirects(&mut network, all_redirects);
    remove_dangling_links(network)
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