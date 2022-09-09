mod stack;
mod word_utils;

use std::iter::once;

use crate::quran::{quran_iter, SIMPLE_CLEAN};
pub use stack::Stack;
use word_utils::WordSuffixIterExt;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new('\0');
    for (s, a, t) in quran_iter(SIMPLE_CLEAN) {
        for (i, t) in t.iter_word_suffix().enumerate() {
            expand_node(&mut root, t, (s, a, i as u8 + 1), word_count_limit);
        }
    }
    root
}

fn expand_node(mut node: &mut Node, text: &str, location: (u8, u16, u8), wcl: u8) {
    let mut word_count = 0;
    for (c, next_c) in text.chars().zip(text.chars().skip(1).chain(once(' '))) {
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
    pub locations: Stack<(u8, u16, u8)>,
}

impl Node {
    fn new(content: char) -> Self {
        Self {
            content,
            next_harfs: Stack::new(),
            locations: Stack::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quran_index() {
        let root = build_quran_index(u8::MAX);
        assert_eq!(root.content, '\0');
        assert_eq!(root.next_harfs.len(), 31);
        assert_eq!(find_next(&root, 'ب').locations.len(), 0);
        assert_eq!(find_next(&root, 'ن').locations.len(), 1);
    }

    fn find_next(node: &Node, target: char) -> &Node {
        node.next_harfs
            .iter()
            .find(|h| h.content == target)
            .unwrap()
    }
}
