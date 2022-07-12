use super::word_utils::WordSuffixIter;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new('\0');
    for (s, a, t) in crate::quran::simple_clean_iter() {
        root.update_tree(s, a, t, word_count_limit);
    }
    root
}

pub struct Node {
    pub content: char,
    pub next_harfs: Vec<Node>,
    pub locations: Vec<(u8, u16, u8)>,
}

impl Node {
    fn new(content: char) -> Self {
        Self {
            content,
            next_harfs: Vec::new(),
            locations: Vec::new(),
        }
    }

    fn update_tree(&mut self, sura_number: u8, aya_number: u16, aya_text: &str, wc_limit: u8) {
        for (i, t) in WordSuffixIter::from(aya_text).enumerate() {
            let location = (sura_number, aya_number, i as u8 + 1);
            let mut node = &mut *self;
            let mut word_count = 0;
            for c in t.chars().chain(std::iter::once(' ')) {
                if word_count >= wc_limit {
                    break;
                }
                node = node.get_or_add(c);
                if node.content == ' ' {
                    word_count += 1;
                    node.locations.push(location);
                }
            }
        }
    }

    fn get_or_add(&mut self, content: char) -> &mut Self {
        let pos = self.next_harfs.iter().position(|h| h.content == content);
        match pos {
            Some(index) => self.next_harfs.get_mut(index).unwrap(),
            None => {
                self.next_harfs.push(Node::new(content));
                self.next_harfs.last_mut().unwrap()
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
        let nun = find_next(&root, 'ن');
        assert_eq!(nun.locations.len(), 0);
        assert_eq!(find_next(nun, ' ').locations, vec![(68, 1, 1)]);
    }

    fn find_next(node: &Node, target: char) -> &Node {
        node.next_harfs
            .iter()
            .find(|h| h.content == target)
            .unwrap()
    }
}
