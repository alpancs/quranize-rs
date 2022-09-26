mod stack;
mod word_utils;

use crate::quran::{self, CleanCharsExt};
use crate::transliterations as trans;
use stack::Stack;
use word_utils::WordSuffixIterExt;

pub type EncodeResults<'a> = Vec<(String, Vec<&'a str>)>;
type Location = (u8, u16, u8);

pub fn build_quran_index(wcl: u8) -> Node {
    let mut root = Node::new('\0');
    for (s, a, t) in quran::iter() {
        for (i, t) in t.word_suffixes().enumerate() {
            expand_node(&mut root, t, (s, a, i as u8 + 1), wcl);
        }
    }
    root
}

fn expand_node(mut node: &mut Node, text: &str, location: Location, wcl: u8) {
    let mut word_count = 0;
    let next_chars = text.clean_chars().skip(1).chain(std::iter::once(' '));
    for (c, next_c) in text.clean_chars().zip(next_chars) {
        node = node.get_or_add(c);
        if next_c == ' ' {
            node.locations.push(location);
            word_count += 1;
            if word_count >= wcl {
                break;
            }
        }
    }
}

pub struct Node {
    pub content: char,
    pub next_harfs: Stack<Node>,
    pub locations: Stack<Location>,
}

impl Node {
    fn new(content: char) -> Self {
        Self {
            content,
            next_harfs: Stack::new(),
            locations: Stack::new(),
        }
    }

    pub fn get(&self, content: char) -> Option<&Self> {
        self.next_harfs.iter().find(|n| n.content == content)
    }

    fn get_or_add(&mut self, content: char) -> &mut Self {
        let pos = self.next_harfs.iter().position(|h| h.content == content);
        match pos {
            Some(index) => self.next_harfs.iter_mut().nth(index).unwrap(),
            None => {
                self.next_harfs.push(Node::new(content));
                self.next_harfs.peek_mut().unwrap()
            }
        }
    }

    pub fn rev_encode<'a>(&'a self, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !self.locations.is_empty() {
            results.push((String::new(), Vec::new()));
        }
        for subnode in self.next_harfs.iter() {
            let prefixes = trans::map(subnode.content);
            let additional_prefixes = trans::contextual_map(self.content, subnode.content);
            for prefix in prefixes.iter().chain(additional_prefixes) {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut subnode.rev_encode_sub(subtext, prefix));
                }
            }
        }
        results
    }

    fn rev_encode_sub<'a>(&'a self, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode(text);
        for (q, e) in results.iter_mut() {
            q.push(self.content);
            e.push(expl);
        }
        results
    }

    pub fn get_locations(&self, quran: &str) -> Option<&Stack<Location>> {
        let mut chars = quran.chars();
        match chars.next() {
            None => Some(&self.locations),
            Some(c) => match self.get(c) {
                None => None,
                Some(subnode) => subnode.get_locations(chars.as_str()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quran_index() {
        let root = build_quran_index(u8::MAX);
        assert_eq!(root.content, '\0');
        assert_eq!(root.next_harfs.len(), 31);
        assert_eq!(root.get('ب').unwrap().locations.len(), 0);
        assert_eq!(root.get('ن').unwrap().locations.len(), 1);
    }
}
