mod harf;

use super::quran::{SIMPLE_CLEAN, SIMPLE_PLAIN};
pub use harf::Harf as Node;
use std::collections::HashMap;

pub type AyaMap = HashMap<(u8, u16), &'static str>;

pub fn build_quran_index(word_count_limit: u8) -> Node {
    let mut root = Node::new('\0');
    for (sura_number, aya_number, aya_text) in get_aya_iterator(SIMPLE_CLEAN) {
        root.update_tree(sura_number, aya_number, aya_text, word_count_limit);
    }
    root
}

pub fn build_aya_map() -> AyaMap {
    let mut aya_map = HashMap::new();
    for (sura_number, aya_number, aya_text) in get_aya_iterator(SIMPLE_PLAIN) {
        aya_map.insert((sura_number, aya_number), aya_text);
    }
    aya_map
}

fn get_aya_iterator(raw: &str) -> impl Iterator<Item = (u8, u16, &str)> {
    let raw = raw.trim_start();
    let basmalah = raw.split('\n').next().unwrap().split('|').nth(2).unwrap();
    let basmalah = basmalah.to_owned() + " ";
    raw.split('\n')
        .take_while(|l| !l.is_empty())
        .map(move |l| split_aya_line(l, &basmalah))
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
        let quran_index = build_quran_index(u8::MAX);
        assert_eq!(quran_index.content, '\0');
        assert_eq!(quran_index.next_harfs.len(), 31);

        let ba = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ب')
            .unwrap();
        assert!(ba.locations.is_empty());

        let nun = quran_index
            .next_harfs
            .iter()
            .find(|h| h.content == 'ن')
            .unwrap();
        assert_eq!(nun.locations, vec![(68, 1, 1)]);
    }
}
