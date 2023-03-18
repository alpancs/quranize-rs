mod stack;

use crate::transliterations as trans;
use stack::Stack;

pub type EncodeResults<'a> = Vec<(String, Vec<&'a str>)>;
pub type Location = (u8, u16, u8);

pub(crate) struct Node {
    pub harf: char,
    pub next_harfs: Stack<Node>,
    pub locations: Vec<Location>,
}

impl Node {
    pub fn new(content: char) -> Self {
        Self {
            harf: content,
            next_harfs: Stack::new(),
            locations: Default::default(),
        }
    }

    pub fn expand(&mut self, quran: &str, location: Location, min_harfs: usize) {
        let mut node = self;
        let next_chars = quran.chars().skip(1).chain(std::iter::once(' '));
        for (i, (c, next_c)) in quran.chars().zip(next_chars).enumerate() {
            node = node.get_or_add(c);
            if next_c == ' ' {
                node.locations.push(location);
                if i + 1 >= min_harfs {
                    break;
                }
            }
        }
    }

    fn get_or_add(&mut self, content: char) -> &mut Self {
        let pos = self.next_harfs.iter().position(|n| n.harf == content);
        match pos {
            Some(index) => self.next_harfs.iter_mut().nth(index).unwrap(),
            None => {
                self.next_harfs.push(Node::new(content));
                self.next_harfs.peek_mut().unwrap()
            }
        }
    }

    pub fn rev_encode(&self, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !self.locations.is_empty() {
            results.push((String::new(), Vec::new()));
        }
        for subnode in self.next_harfs.iter() {
            let prefixes = trans::map(subnode.harf)
                .iter()
                .chain(trans::contextual_map(self.harf, subnode.harf));
            for prefix in prefixes {
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
            q.push(self.harf);
            e.push(expl);
        }
        results
    }

    pub fn rev_encode_first_aya(&self, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && self.containing_first_aya() {
            results.push((String::new(), Vec::new()));
        }
        for subnode in self.next_harfs.iter() {
            for prefix in trans::single_harf_map(subnode.harf) {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut subnode.rev_encode_sub_first_aya(subtext, prefix));
                }
            }
        }
        results
    }

    fn containing_first_aya(&self) -> bool {
        self.locations.iter().any(|&(_, a, _)| a == 1)
    }

    fn rev_encode_sub_first_aya<'a>(&'a self, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode_first_aya(text);
        for (q, e) in results.iter_mut() {
            q.push(self.harf);
            e.push(expl);
        }
        results
    }

    pub fn get_locations(&self, quran: &str) -> Option<&[Location]> {
        let mut chars = quran.chars();
        match chars.next() {
            Some(c) => self.get(c).and_then(|n| n.get_locations(chars.as_str())),
            None => Some(&self.locations),
        }
    }

    pub fn get(&self, content: char) -> Option<&Self> {
        self.next_harfs.iter().find(|n| n.harf == content)
    }
}
