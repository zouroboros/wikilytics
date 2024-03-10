use std::{io::Result, path::PathBuf};

use crate::common::{find_entries, read_from};

pub fn wikitext(xml_dump_path: &PathBuf, xml_dump_index_path: &PathBuf, title: &String) -> Result<()> {

    let entries = find_entries(xml_dump_index_path, title)?;

    if entries.len() == 0 {
        println!("No article with title {title} found!");
    }

    for entry in entries {

        let pages = read_from(xml_dump_path, entry.start)?;
        
        for page in pages {
            
            if page.title == entry.title {
                let text = page.text.unwrap_or("No wikitext!".to_owned());
                println!("{}", text);
                break;
            }
        }
    }

    Ok(())
}