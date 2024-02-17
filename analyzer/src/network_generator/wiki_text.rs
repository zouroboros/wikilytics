use std::collections::VecDeque;

use parse_wiki_text::{Configuration, Node};

use crate::network_generator::wiki_xml_dump::WikiPage;

type WikiText<'a> = Vec<Node<'a>>;

pub fn parse_text<'a>(page: &'a WikiPage) -> Option<WikiText<'a>> {
    let parser = Configuration::default();
    let text = page.text.as_ref()?;

    let result = parser.parse(text);

    Some(result.nodes)
}

pub fn linked_articles(text: WikiText) -> Vec<String> {
    let mut nodes: VecDeque<Node> = text.into();
    let mut links: VecDeque<String> = VecDeque::new();

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
            Node::Link{target, .. } => links.push_back(target.to_string()),
            Node::OrderedList{ items, .. } => {
                for item in items {
                    nodes.append(&mut item.nodes.into())
                }
            },
            Node::Preformatted{ nodes: pre_nodes, .. } => nodes.append(&mut pre_nodes.into()),
            Node::Redirect{ target, .. } => links.push_back(target.to_string()),
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

pub fn is_redirect(text: &WikiText) -> bool {
    text.iter().any(|node| { match node {
        Node::Redirect{..} => true,
        _ => false
    } })
}

pub fn redirects_to(text: &WikiText) -> Option<String> {
    return text.iter().map(|node| 
        match *node {
            Node::Redirect{ target, .. } => Some(target.to_owned()),
            _ => None
    }).find(|option| { 
        match option {
            Some(..) => true,
            _ => false
    } })?;
}