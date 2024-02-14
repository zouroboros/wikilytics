use std::path::PathBuf;

mod network_generator;
mod statistics;
mod network;

use crate::network::network;

fn main() -> std::io::Result<()> {
    println!("wikilytics");
    let cmd = clap::Command::new("wikilytics")
        .bin_name("wikilytics")
        .subcommand_required(true)
        .subcommand(clap::command!("network")
            .arg(clap::arg!(<XMLDUMPFILE> "Path to the wikipedia xml dump")
            .value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::arg!(<NETWORKFILE> "Where to save the network")
            .value_parser(clap::value_parser!(PathBuf))));

    if let Some(("network", matches)) = cmd.get_matches().subcommand() {
        let wiki_xml_dump_path = matches.get_one::<PathBuf>("XMLDUMPFILE").unwrap();
        let network_file_path = matches.get_one::<PathBuf>("NETWORKFILE").unwrap();
        network(wiki_xml_dump_path.to_owned(), network_file_path.to_owned())?;
        println!("{wiki_xml_dump_path:?}")
    }

    Ok(())
}
