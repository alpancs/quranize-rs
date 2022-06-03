use std::collections::HashMap;

use wasm_bindgen::prelude::*;

mod quran_index;
use quran_index::{build_aya_index, build_quran_index, Harf};

mod transliteration_map;
use transliteration_map::{build_transliteration_map, TransliterationMap};

type EncodeResults = Vec<(String, Vec<(u8, u16, u8)>)>;

#[wasm_bindgen]
pub struct Quranize {
    quran_index: Harf,
    transliteration_map: TransliterationMap,
    aya_index: HashMap<(u8, u16), String>,
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
            aya_index: build_aya_index(),
        }
    }

    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(&self.quran_index, &normalize(text));
        results.dedup_by(|(q1, _), (q2, _)| q1 == q2);
        results
            .into_iter()
            .map(|(q, ls)| (q.chars().rev().collect(), ls))
            .collect()
    }

    fn rev_encode(&self, node: &Harf, text: &str) -> EncodeResults {
        let mut results = vec![];
        if text.is_empty() && !node.locations.is_empty() {
            results.push((String::new(), node.locations.clone()));
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

    fn rev_encode_subnode(&self, subnode: &Harf, subtext: &str) -> EncodeResults {
        let mut results = self.rev_encode(subnode, subtext);
        for (q, _) in results.iter_mut() {
            q.push(subnode.content);
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
        let encode_results: Vec<JsEncodeResult> = self
            .encode(text)
            .into_iter()
            .map(|(q, ls)| {
                let word_count = q.split_whitespace().count();
                JsEncodeResult {
                    quran: q,
                    locations: ls
                        .into_iter()
                        .map(|(s, a, w)| {
                            let w = w as usize;
                            let aya_text = self.aya_index.get(&(s, a)).unwrap();
                            JsLocation {
                                sura_number: s,
                                aya_number: a,
                                before_text: get_subword(aya_text, 0, w - 1),
                                text: get_subword(aya_text, w - 1, word_count),
                                after_text: get_subword(aya_text, w - 1 + word_count, usize::MAX),
                            }
                        })
                        .collect(),
                }
            })
            .collect();
        JsValue::from_serde(&encode_results).unwrap()
    }
}

fn get_subword(text: &str, n_skip: usize, n_take: usize) -> String {
    text.split_whitespace()
        .skip(n_skip)
        .take(n_take)
        .collect::<Vec<_>>()
        .join(" ")
}

#[derive(serde::Serialize)]
struct JsEncodeResult {
    quran: String,
    locations: Vec<JsLocation>,
}

#[derive(serde::Serialize)]
struct JsLocation {
    sura_number: u8,
    aya_number: u16,
    before_text: String,
    text: String,
    after_text: String,
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
        assert_eq!(quranize.encode("bismillah")[0].1.len(), 3);
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
        quranize.encode(text).into_iter().map(|(q, _)| q).collect()
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
