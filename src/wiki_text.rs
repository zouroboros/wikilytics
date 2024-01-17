use std::collections::VecDeque;

use parse_wiki_text::{Configuration, Node};

use crate::wiki_pages::WikiPage;

pub fn linked_articles<'a>(page: &'a WikiPage) -> Vec<&str> {
    let parser = Configuration::default();
    let result = parser.parse(&page.text);

    filter_links(result.nodes)
}

fn filter_links(nodes: Vec<Node>) -> Vec<&str> {
    let mut nodes = VecDeque::from(nodes);
    let mut links: VecDeque<&str> = VecDeque::new();

    while let Some(node) = nodes.pop_front() {
        match node {
            Node::Category{ ordinal, .. } => nodes.append(&mut ordinal.into()),
            Node::DefinitionList{ items, .. } => {
                for item in items {
                    nodes.append(&mut item.nodes.into())
                }
            },
            Node::Heading{ nodes: heading_nodes, .. } => nodes.append(&mut heading_nodes.into()),
            Node::Image{ text, ..} => nodes.append(&mut text.into()),
            Node::Link{target, .. } => links.push_back(target),
            Node::OrderedList{ items, .. } => {
                for item in items {
                    nodes.append(&mut item.nodes.into())
                }
            },
            Node::Preformatted{ nodes: pre_nodes, .. } => nodes.append(&mut pre_nodes.into()),
            Node::Redirect{ target, .. } => links.push_back(target),
            Node::Table{ attributes, rows, .. } => {
                nodes.append(&mut attributes.into());
                for row in rows {
                    nodes.append(&mut row.attributes.into());
                    for cell in row.cells {
                        nodes.append(&mut cell.content.into());
                    }
                }
            },
            Node::Tag{ nodes: tag_nodes, .. } => nodes.append(&mut tag_nodes.into()),
            Node::UnorderedList{ items, .. } => {
                for item in items {
                    nodes.append(&mut item.nodes.into());
                }
            }
            _ => (),
        }
    }

    links.into()
}
