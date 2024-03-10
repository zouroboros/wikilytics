use std::path::PathBuf;

mod network_generator;
mod statistics;
mod network;
mod analyze;
mod wikitext;
mod common;
mod resolve;

use crate::network::network;
use crate::analyze::analyze;
use crate::wikitext::wikitext;
use crate::resolve::resolve;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let cmd = clap::Command::new("wikilytics")
        .bin_name("wikilytics")
        .subcommand_required(true)
        .subcommand(clap::command!("resolve")
            .arg(clap::arg!(<XMLDUMPFILE> "Path to the wikipedia xml dump")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<XMLDUMPINDEXFILE> "Path to the wikipedia xml dump index file")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<TITLE> "Title of the article")
            .value_parser(clap::value_parser!(String))))
        .subcommand(clap::command!("wikitext")
            .arg(clap::arg!(<XMLDUMPFILE> "Path to the wikipedia xml dump")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<XMLDUMPINDEXFILE> "Path to the wikipedia xml dump index file")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<TITLE> "Title of the article")
            .value_parser(clap::value_parser!(String))))
        .subcommand(clap::command!("network")
            .arg(clap::arg!(<XMLDUMPFILE> "Path to the wikipedia xml dump")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<XMLDUMPINDEXFILE> "Path to the wikipedia xml dump index file")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<NETWORKFILE> "Where to save the network")
            .value_parser(clap::value_parser!(PathBuf))))
        .subcommand(clap::command!("analyze")
            .arg(clap::arg!(<NETWORKFILE> "Path to the network file")
                .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<STATISTICSFILE>)
                .value_parser(clap::value_parser!(PathBuf))));

    let matches = cmd.get_matches();
    let subcommand = matches.subcommand();

    if let Some(("network", matches)) = subcommand {
        let wiki_xml_dump_path = matches.get_one::<PathBuf>("XMLDUMPFILE").unwrap();
        let wiki_xml_dump_index_path = matches.get_one::<PathBuf>("XMLDUMPINDEXFILE").unwrap().to_owned();
        let network_file_path = matches.get_one::<PathBuf>("NETWORKFILE").unwrap();
        network(wiki_xml_dump_path.to_owned(), wiki_xml_dump_index_path, network_file_path.to_owned())?;
    }

    if let Some(("analyze", matches)) = subcommand {
        let network_file_path = matches.get_one::<PathBuf>("NETWORKFILE").unwrap();
        let statistics_file_path = matches.get_one::<PathBuf>("STATISTICSFILE").unwrap();
        analyze(network_file_path.to_owned(), statistics_file_path.to_owned())?;
    }

    if let Some(("wikitext", matches)) = subcommand {
        let wiki_xml_dump_path = matches.get_one::<PathBuf>("XMLDUMPFILE").unwrap();
        let wiki_xml_dump_index_path = matches.get_one::<PathBuf>("XMLDUMPINDEXFILE").unwrap();
        let article_title = matches.get_one::<String>("TITLE").unwrap();

        wikitext(wiki_xml_dump_path, wiki_xml_dump_index_path, article_title)?;
    }

    if let Some(("resolve", matches)) = subcommand {
        let wiki_xml_dump_path = matches.get_one::<PathBuf>("XMLDUMPFILE").unwrap();
        let wiki_xml_dump_index_path = matches.get_one::<PathBuf>("XMLDUMPINDEXFILE").unwrap();
        let article_title = matches.get_one::<String>("TITLE").unwrap();

        resolve(wiki_xml_dump_path, wiki_xml_dump_index_path, article_title).unwrap();
    }

    Ok(())
}
