#![feature(test)]

use serde::Serialize;
use wasm_bindgen::prelude::*;

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod transliteration_map;
use transliteration_map::{build_transliteration_map, TransliterationMap};

pub fn build_quranize() -> Quranize {
    Quranize {
        quran_index: build_quran_index(),
        transliteration_map: build_transliteration_map(),
    }
}

#[wasm_bindgen]
pub struct Quranize {
    quran_index: Harf,
    transliteration_map: TransliterationMap,
}

impl Quranize {
    pub fn encode(&self, text: &str) -> Vec<EncodeResult> {
        self.encode_with_context(&self.quran_index, &normalize(text))
    }

    fn encode_with_context(&self, node: &Harf, text: &str) -> Vec<EncodeResult> {
        match (text, &node.locations) {
            ("", locations) if locations.is_empty() => vec![],
            ("", locations) => vec![EncodeResult::new(locations)],
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

    fn encode_subnode(&self, subnode: &Harf, subtext: &str) -> Vec<EncodeResult> {
        self.encode_with_context(subnode, subtext)
            .into_iter()
            .map(|r| EncodeResult {
                quran: subnode.content.to_string() + &r.quran,
                ..r
            })
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

#[derive(Serialize)]
pub struct EncodeResult {
    quran: String,
    locations: Vec<Location>,
}

#[derive(Serialize)]
pub struct Location {
    sura_number: u8,
    aya_number: u16,
    word_number: u8,
}

impl EncodeResult {
    fn new(locations: &[(u8, u16, u8)]) -> Self {
        Self {
            quran: "".to_string(),
            locations: locations
                .iter()
                .map(|&(sura_number, aya_number, word_number)| Location {
                    sura_number,
                    aya_number,
                    word_number,
                })
                .collect(),
        }
    }
}

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Quranize {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        build_quranize()
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn js_encode(&self, text: &str) -> JsValue {
        JsValue::from_serde(&self.encode(text)).unwrap()
    }
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
        assert_eq!(quranize.encode("bismillah")[0].locations.len(), 3);
        assert_eq!(get_encoded_quran(&quranize, "bisyimaalihi"), vec!["بشماله"]);
        assert_eq!(
            get_encoded_quran(&quranize, "bismilla hirrohmaan nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
    }

    fn get_encoded_quran(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|r| r.quran).collect()
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
