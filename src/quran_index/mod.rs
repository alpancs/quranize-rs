use crate::transliterations as trans;

pub type EncodeResults<'a> = Vec<(String, Vec<&'a str>)>;
pub type Location = (u8, u16, u8);

pub(crate) struct Node {
    pub harf: char,
    pub childs: Option<Box<Nodes>>,
    pub locations: Vec<Location>,
}

pub(crate) struct Nodes {
    head: Node,
    tail: Option<Box<Nodes>>,
}

impl Node {
    pub fn new(content: char) -> Self {
        Self {
            harf: content,
            childs: Default::default(),
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

    fn get_or_add(&mut self, harf: char) -> &mut Self {
        let pos = self.iter().position(|n| n.harf == harf);
        match pos {
            Some(index) => self.iter_mut().nth(index).unwrap(),
            None => {
                self.childs = Some(Box::new(Nodes {
                    head: Node::new(harf),
                    tail: self.childs.take(),
                }));
                &mut self.childs.as_mut().unwrap().head
            }
        }
    }

    pub fn rev_encode(&self, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !self.locations.is_empty() {
            results.push((String::new(), Vec::new()));
        }
        for subnode in self.iter() {
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
        for subnode in self.iter() {
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

    pub fn get(&self, harf: char) -> Option<&Self> {
        self.iter().find(|n| n.harf == harf)
    }

    fn iter(&self) -> Iter {
        Iter {
            childs: self.childs.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut {
            childs: self.childs.as_deref_mut(),
        }
    }

    #[cfg(test)]
    pub fn child_count(&self) -> usize {
        self.iter().count()
    }
}

struct Iter<'a> {
    childs: Option<&'a Nodes>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        self.childs.map(|nodes| {
            self.childs = nodes.tail.as_deref();
            &nodes.head
        })
    }
}

struct IterMut<'a> {
    childs: Option<&'a mut Nodes>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        self.childs.take().map(|nodes| {
            self.childs = nodes.tail.as_deref_mut();
            &mut nodes.head
        })
    }
}
