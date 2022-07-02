const ARABIC_UNICODE_START: u32 = 0x0600;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new(' ');
    for (s, a, t) in crate::quran::simple_clean_iter() {
        root.update_tree(s, a, t, word_count_limit);
    }
    root
}

pub struct Node {
    pub content_code: u8,
    pub next_harfs: Vec<Node>,
    pub locations: Vec<(u8, u16, u8)>,
}

impl Node {
    fn new(content: char) -> Self {
        Self {
            content_code: char_to_code(content),
            next_harfs: Vec::new(),
            locations: Vec::new(),
        }
    }

    pub fn content(&self) -> char {
        code_to_char(self.content_code)
    }

    fn update_tree(&mut self, sura_number: u8, aya_number: u16, aya_text: &str, wc_limit: u8) {
        let mut word_number = 0;
        let aya_chars = Vec::from_iter(aya_text.chars());
        for i in 0..aya_chars.len() {
            if i == 0 || aya_chars[i - 1] == ' ' {
                word_number += 1;
                let mut node = &mut *self;
                let mut word_count = 0;
                for j in i..aya_chars.len() {
                    node = node.get_or_add(aya_chars[j]);
                    if j == aya_chars.len() - 1 || aya_chars[j + 1] == ' ' {
                        word_count += 1;
                        if word_count > wc_limit {
                            break;
                        }
                        node.locations.push((sura_number, aya_number, word_number));
                    }
                }
            }
        }
    }

    fn get_or_add(&mut self, content: char) -> &mut Self {
        let pos = self.next_harfs.iter().position(|h| h.content() == content);
        match pos {
            Some(index) => self.next_harfs.get_mut(index).unwrap(),
            None => {
                self.next_harfs.push(Node::new(content));
                self.next_harfs.last_mut().unwrap()
            }
        }
    }
}

fn char_to_code(c: char) -> u8 {
    match c {
        ' ' => 0,
        _ => (u32::from(c) - ARABIC_UNICODE_START).try_into().unwrap(),
    }
}

fn code_to_char(c: u8) -> char {
    match c {
        0 => ' ',
        _ => char::try_from(ARABIC_UNICODE_START + u32::from(c)).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quran_index() {
        let root = build_quran_index(u8::MAX);
        assert_eq!(root.content_code, 0);
        assert_eq!(root.next_harfs.len(), 31);
        assert!(find_next(&root, 'ب').locations.is_empty());
        assert_eq!(find_next(&root, 'ن').locations, vec![(68, 1, 1)]);
    }

    fn find_next(node: &Node, target: char) -> &Node {
        node.next_harfs
            .iter()
            .find(|h| h.content() == target)
            .unwrap()
    }

    #[test]
    fn test_char_to_code() {
        assert_eq!(char_to_code(' '), 0);
        assert_eq!(char_to_code('ء'), 0x21);
        assert_eq!(char_to_code('ي'), 0x4a);
    }

    #[test]
    fn test_code_to_char() {
        assert_eq!(code_to_char(0), ' ');
        assert_eq!(code_to_char(0x21), 'ء');
        assert_eq!(code_to_char(0x4a), 'ي');
    }
}
