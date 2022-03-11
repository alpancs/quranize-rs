#![feature(test)]

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod transliteration_map;
use transliteration_map::{build_transliteration_map, TransliterationMap};

pub struct Quranize {
    quran_index: Harf,
    transliteration_map: TransliterationMap,
}

pub fn build_quranize() -> Quranize {
    Quranize {
        quran_index: build_quran_index(),
        transliteration_map: build_transliteration_map(),
    }
}

type EncodeResult = Vec<(String, Vec<(u8, u16, u8)>)>;

impl Quranize {
    pub fn encode(&self, text: &str) -> EncodeResult {
        self.encode_with_context(&self.quran_index, &normalize(text))
    }

    fn encode_with_context(&self, node: &Harf, text: &str) -> EncodeResult {
        match (text, &node.locations) {
            ("", locations) if locations.is_empty() => vec![],
            ("", locations) => vec![("".to_string(), locations.to_vec())],
            _ => {
                let mut results = vec![];
                for subnode in node.next_harfs.iter() {
                    for prefix in self.transliteration_map[&subnode.content].iter() {
                        if let Some(subtext) = text.strip_prefix(prefix) {
                            results.append(&mut self.encode_subnode(subnode, subtext));
                        }
                    }
                    if node.content == 'ا' && subnode.content == 'ل' {
                        results.append(&mut self.encode_subnode(subnode, text));
                    }
                }
                results
            }
        }
    }

    fn encode_subnode(&self, subnode: &Harf, subtext: &str) -> EncodeResult {
        self.encode_with_context(subnode, subtext)
            .into_iter()
            .map(|(q, l)| (subnode.content.to_string() + &q, l))
            .collect()
    }
}

fn normalize(text: &str) -> String {
    let mut text: Vec<_> = text
        .chars()
        .filter_map(|c| match c.to_ascii_lowercase() {
            c @ ('a'..='z' | '\'') => Some(c),
            _ => None,
        })
        .collect();
    text.dedup_by(|a, b| a == b && *a != 'l');
    String::from_iter(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_quranize_normal() {
        let quranize = build_quranize();
        assert_eq!(
            get_encoded_quran(&quranize, "bismi"),
            vec!["باسم", "بعصم", "بئسما", "بإثمي", "بسم"]
        );
        assert_eq!(get_encoded_quran(&quranize, "bismillah"), vec!["بسم الله"]);
        assert_eq!(
            quranize.encode("bismillah").first().unwrap().1,
            vec![(1, 1, 1), (11, 41, 4), (27, 30, 5)]
        );
        assert_eq!(get_encoded_quran(&quranize, "bisyimaalihi"), vec!["بشماله"]);
        assert_eq!(
            get_encoded_quran(&quranize, "bismilla hirrohmaan nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
    }

    fn get_encoded_quran(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|(q, _)| q).collect()
    }

    #[test]
    fn test_quranize_zero() {
        let quranize = build_quranize();
        assert!(quranize.encode("").is_empty());
        assert!(quranize.encode("gsquw").is_empty());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("bismi"), "bismi");
        assert_eq!(normalize("'aalimul ghoibi"), "'alimulghoibi");
        assert_eq!(normalize("Qul A'udzu"), "qula'udzu");
    }

    #[bench]
    fn bench_quranize(b: &mut Bencher) {
        let quranize = build_quranize();
        b.iter(|| quranize.encode("bismillah"));
    }
}
