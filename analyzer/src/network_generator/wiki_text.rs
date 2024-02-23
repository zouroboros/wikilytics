use std::cmp::min;

use crate::network_generator::wiki_xml_dump::WikiPage;

#[derive(PartialEq, Debug)]
pub enum LinkOrRedirect {
    Link(String), Redirect(String)
}

impl LinkOrRedirect {
    pub fn is_link(&self) -> bool {
        match self {
            LinkOrRedirect::Link(_) => true,
            LinkOrRedirect::Redirect(_) => false
        }
    }
    
    pub fn is_redirect(&self) -> bool {
        match self {
            LinkOrRedirect::Link(_) => false,
            LinkOrRedirect::Redirect(_) => true
        }
    }
    
    pub fn link_text(&self) -> Option<&String> {
        match self {
            LinkOrRedirect::Link(link) => Some(link),
            LinkOrRedirect::Redirect(_) => None
        }
    }
    
    pub fn redirect_text(&self) -> Option<&String> {
        match self {
            LinkOrRedirect::Link(_) => None,
            LinkOrRedirect::Redirect(link) => Some(link)
        }
    }    
}

pub fn parse_text(page: &WikiPage) -> Option<Vec<LinkOrRedirect>> {
    let text = page.text.as_ref()?;
    let chars = text.chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut links_or_redirects = vec![];

    let redirect_token_uppercase = ['#', 'R', 'E', 'D', 'I', 'R', 'E', 'C', 'T'];
    let redirect_token_lowercase = ['#', 'r', 'e', 'd', 'i', 'r', 'e', 'c', 't'];
    let start_link_token = ['[', '['];
    let end_link_token = [']', ']'];

    let read_link = |start_index: usize| {
        
        let mut current_index = start_index;
    
        while current_index + end_link_token.len() <= chars.len() && 
            chars[current_index..current_index + end_link_token.len()] != end_link_token {
            current_index += 1;
        }

        let link_chars = &chars[start_index..current_index];

        (link_chars.iter().take_while(|c| **c != '|').collect::<String>(), current_index + end_link_token.len())
    };

    let check_prefix = |index: usize, token: &[char]| {
        let end_index = min(chars.len(), index + token.len());
        chars[index..end_index] == *token
    };

    let check_link_start = |index| {
        check_prefix(index, &start_link_token) && chars.len() > index + start_link_token.len()
    };

    let check_redirect_start = |index| {
        (check_prefix(index, &redirect_token_uppercase) ||
            check_prefix(index, &redirect_token_lowercase))
        && chars.len() > index + redirect_token_uppercase.len()
    };

    while index < chars.len() {
        if check_link_start(index) {
            let (link_text, next_index) = read_link(index + start_link_token.len());
            links_or_redirects.push(LinkOrRedirect::Link(link_text));
            index = next_index;
        } else if check_redirect_start(index) {
            let (link_text, next_index) = read_link(index + redirect_token_uppercase.len() + 1 + start_link_token.len());
            links_or_redirects.push(LinkOrRedirect::Redirect(link_text));
            index = next_index;
        } else {
            index += 1;
        }
    }
    

    Some(links_or_redirects)
}

pub fn linked_articles<'a>(text: &'a Vec<LinkOrRedirect>) -> Vec<&'a String> {
    text.iter()
        .filter_map(|link| link.link_text())
        .collect::<Vec<&String>>()
}

pub fn is_redirect(text: &Vec<LinkOrRedirect>) -> bool {
    text.iter().any(|link| link.is_redirect())
}

pub fn redirects_to<'a>(text: &'a Vec<LinkOrRedirect>) -> Option<&'a String> {
    text.iter().filter_map(|link| link.redirect_text()).next()
}

#[cfg(test)]
mod tests {
    use crate::network_generator::{wiki_text::LinkOrRedirect, wiki_xml_dump::WikiPage};

    use super::parse_text;


    #[test]
    fn test_parse_empty_page() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: None,
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, None);
    }

    #[test]
    fn test_parse_page_without_text() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![]));
    }

    #[test]
    fn test_parse_page_without_links() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("April, April".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![]));
    }

    #[test]
    fn test_parse_page_with_link() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("[[Link]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Link("Link".to_string())]));
    }

    #[test]
    fn test_parse_page_with_renamed_link() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("[[Link|Other text]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Link("Link".to_string())]));
    }

    #[test]
    fn test_parse_page_with_redirect() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("#REDIRECT [[Link]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Redirect("Link".to_string())]));
    }

    #[test]
    fn test_parse_page_with_redirect_lowercase() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("#redirect [[Link]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Redirect("Link".to_string())]));
    }

    #[test]
    fn test_parse_page_with_two_redirect() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("#REDIRECT [[Link1]]\n#REDIRECT [[Link2]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Redirect("Link1".to_string()), 
            LinkOrRedirect::Redirect("Link2".to_string())]));
    }

    #[test]
    fn test_parse_page_with_renamed_redirect() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("#REDIRECT [[Link|Another name]]".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Redirect("Link".to_string())]));
    }

    #[test]
    fn test_parse_page_with_unclosed_brackets() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("[[ ".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![LinkOrRedirect::Link("".to_string())]));
    }

    #[test]
    fn test_parse_page_with_broken_redirect() {
        let test_page = WikiPage{
            namespace_id: 1,
            text: Some("#REDIRECT".to_string()),
            title: "Test".to_string()
        };

        let parse_result = parse_text(&test_page);

        assert_eq!(parse_result, Some(vec![]));
    }
}