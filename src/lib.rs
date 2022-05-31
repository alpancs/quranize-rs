use serde::Serialize;
use wasm_bindgen::prelude::*;

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod transliteration_map;
use transliteration_map::{build_transliteration_map, TransliterationMap};

#[wasm_bindgen]
pub struct Quranize {
    quran_index: Harf,
    transliteration_map: TransliterationMap,
}

impl Default for Quranize {
    fn default() -> Self {
        Self::new(u8::MAX)
    }
}

impl Quranize {
    pub fn new(word_count_limit: u8) -> Self {
        Self {
            quran_index: build_quran_index(word_count_limit),
            transliteration_map: build_transliteration_map(),
        }
    }

    pub fn encode(&self, text: &str) -> Vec<EncodeResult> {
        let mut results = self.rev_encode(&self.quran_index, &normalize(text));
        for r in results.iter_mut() {
            r.quran = r.quran.chars().rev().collect();
        }
        results.dedup();
        results
    }

    fn rev_encode(&self, node: &Harf, text: &str) -> Vec<EncodeResult> {
        let mut results = vec![];
        if text.is_empty() && !node.locations.is_empty() {
            results.push(EncodeResult::new(&node.locations));
        }
        for subnode in node.next_harfs.iter() {
            for prefix in self.transliteration_map[&subnode.content].iter() {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_subnode(subnode, subtext));
                }
            }
            if node.content == 'ا' && subnode.content == 'ل' {
                results.append(&mut self.rev_encode_subnode(subnode, text));
            }
            if node.content == 'و' && subnode.content == 'ا' {
                results.append(&mut self.rev_encode_subnode(subnode, text));
            }
        }
        results
    }

    fn rev_encode_subnode(&self, subnode: &Harf, subtext: &str) -> Vec<EncodeResult> {
        let mut results = self.rev_encode(subnode, subtext);
        for r in results.iter_mut() {
            r.quran.push(subnode.content);
        }
        results
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
    text.dedup_by(|a, b| a == b && *a != 'l' && *a != 'a' && *a != 'i' && *a != 'u');
    String::from_iter(text)
}

#[derive(Serialize, PartialEq, Eq)]
pub struct EncodeResult {
    quran: String,
    locations: Vec<Location>,
}

#[derive(Serialize, PartialEq, Eq)]
pub struct Location {
    sura_number: u8,
    aya_number: u16,
    word_number: u8,
}

impl EncodeResult {
    fn new(locations: &[(u8, u16, u8)]) -> Self {
        Self {
            quran: String::new(),
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
    #[wasm_bindgen(constructor)]
    pub fn js_new(word_count_limit: u8) -> Self {
        if word_count_limit == 0 {
            Default::default()
        } else {
            Quranize::new(word_count_limit)
        }
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn js_encode(&self, text: &str) -> JsValue {
        JsValue::from_serde(&self.encode(text)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quranize_normal() {
        let quranize: Quranize = Default::default();
        assert_eq!(
            get_encoded_quran(&quranize, "bismi"),
            vec!["باسم", "بعصم", "بئسما", "بإثمي", "بسم"]
        );
        assert_eq!(
            get_encoded_quran(&quranize, "bismillah"),
            vec!["بسم الله", "بشماله"]
        );
        assert_eq!(quranize.encode("bismillah")[0].locations.len(), 3);
        assert_eq!(
            get_encoded_quran(&quranize, "bismilla hirrohman nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
        assert_eq!(
            get_encoded_quran(&quranize, "alhamdulilla hirobbil 'alamiin"),
            vec!["الحمد لله رب العالمين"]
        );
        assert_eq!(
            get_encoded_quran(&quranize, "wa'tasimu bihablillah"),
            vec!["واعتصموا بحبل الله"]
        );
        assert_eq!(
            get_encoded_quran(&quranize, "inna anzalna"),
            vec!["إنا أنزلنا"]
        );
    }

    fn get_encoded_quran(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|r| r.quran).collect()
    }

    #[test]
    fn test_quranize_zero() {
        let quranize: Quranize = Default::default();
        assert!(quranize.encode("").is_empty());
        assert!(quranize.encode("bbb").is_empty());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("bismi"), "bismi");
        assert_eq!(
            normalize("bismilla hirrohman nirrohiim"),
            "bismillahirohmanirohiim"
        );
        assert_eq!(normalize("'aalimul ghoibi"), "'aalimulghoibi");
        assert_eq!(normalize("Qul A'udzu"), "qula'udzu");
    }
}
