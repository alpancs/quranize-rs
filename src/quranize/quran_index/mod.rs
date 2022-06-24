mod harf;

use std::collections::HashMap;

use super::quran::{SIMPLE_CLEAN, SIMPLE_PLAIN};
pub use harf::HarfNode as Node;

pub type AyaMap = HashMap<(u8, u16), &'static str>;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new('\0');
    iterate_quran(SIMPLE_CLEAN, |(s, a, t)| {
        root.update_tree(s, a, t, word_count_limit);
    });
    root
}

pub fn build_aya_simple_plain_map() -> AyaMap {
    let mut aya_map = HashMap::new();
    iterate_quran(SIMPLE_PLAIN, |(s, a, t)| {
        aya_map.insert((s, a), t);
    });
    aya_map
}

fn iterate_quran<'a, F>(raw: &'a str, f: F)
where
    F: FnMut((u8, u16, &'a str)),
{
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    let basmalah = basmalah.to_owned() + " ";
    raw.split('\n')
        .take_while(|l| !l.is_empty())
        .map(|l| split_aya_line(l, &basmalah))
        .for_each(f);
}

fn split_aya_line<'a>(line: &'a str, basmalah: &str) -> (u8, u16, &'a str) {
    let mut line_parts = line.split('|');
    let sura_number = line_parts.next().unwrap().parse().unwrap();
    let aya_number = line_parts.next().unwrap().parse().unwrap();
    let aya_text = line_parts.next().unwrap().trim_start_matches(basmalah);
    (sura_number, aya_number, aya_text)
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
