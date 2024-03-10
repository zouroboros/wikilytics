use std::{io::Result, path::PathBuf};

use crate::{common::{find_entries, read_from}, network_generator::wiki_text::parse_text};

pub fn resolve(xml_dump_path: &PathBuf, xml_dump_index_path: &PathBuf, title: &String) -> Result<()> {
    
    let entries = find_entries(xml_dump_index_path, title)?;

    for entry in entries {
        let pages = read_from(xml_dump_path, entry.start)?;
        
        for page in pages {

            if page.title == *title {
                
                if let Some(links_or_redirects) = parse_text(&page) {
                    
                    let redirects = links_or_redirects.iter()
                        .filter_map(|link_or_redirect| link_or_redirect.redirect_text());
                    
                    for redirect in redirects {
                        println!("{}", redirect);
                    }
                }

                break;
            }
        }
    }

    Ok(())
}