mod harf;

pub use harf::HarfNode as Node;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new('\0');
    for (s, a, t) in crate::quran::simple_clean_iter() {
        root.update_tree(s, a, t, word_count_limit);
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quran_index() {
        let root = build_quran_index(u8::MAX);
        assert_eq!(root.content, '\0');
        assert_eq!(root.next_harfs.len(), 31);
        assert!(find_next(&root, 'ب').locations.is_empty());
        assert_eq!(find_next(&root, 'ن').locations, vec![(68, 1, 1)]);
    }

    fn find_next(node: &Node, target: char) -> &Node {
        node.next_harfs
            .iter()
            .find(|h| h.content == target)
            .unwrap()
    }
}
