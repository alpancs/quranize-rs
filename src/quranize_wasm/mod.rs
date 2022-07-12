mod aya_map;

use wasm_bindgen::prelude::*;

use crate::Quranize;

#[wasm_bindgen(js_name = Quranize)]
pub struct JsQuranize {
    quranize: Quranize,
    aya_map: aya_map::Map,
}

#[wasm_bindgen(js_class = Quranize)]
impl JsQuranize {
    #[wasm_bindgen(constructor)]
    pub fn new(word_count_limit: u8) -> Self {
        Self {
            quranize: match word_count_limit {
                0 => Quranize::default(),
                n => Quranize::new(n),
            },
            aya_map: aya_map::build_map(),
        }
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn js_encode(&self, text: &str) -> JsValue {
        JsValue::from_serde(&self.encode(text)).unwrap()
    }

    fn encode<'a>(&'a self, text: &'a str) -> Vec<JsEncodeResult<'a>> {
        self.quranize
            .encode(text)
            .into_iter()
            .map(|(quran, locations, explanations)| JsEncodeResult {
                locations: self.to_js_locations(&quran, locations),
                quran,
                explanations,
            })
            .collect()
    }

    fn to_js_locations(&self, quran: &str, locations: &[(u8, u16, u8)]) -> Vec<JsLocation> {
        let word_count = quran.split_whitespace().count();
        locations
            .iter()
            .map(|&(sura_number, aya_number, word_number)| {
                let text = self.aya_map.get(sura_number, aya_number).unwrap();
                let (before_text, text) = split_at_nth_word(text, word_number as usize - 1);
                let (text, after_text) = split_at_nth_word(text, word_count);
                JsLocation {
                    sura_number,
                    aya_number,
                    before_text,
                    text,
                    after_text,
                }
            })
            .collect()
    }
}

fn split_at_nth_word(text: &str, n: usize) -> (&str, &str) {
    let mut split_index = None;
    let mut char_indices = text.char_indices();
    for _ in 0..n {
        loop {
            match char_indices.next() {
                Some((i, ' ')) => {
                    split_index = Some(i);
                    break;
                }
                None => {
                    split_index = Some(text.len());
                    break;
                }
                _ => {}
            }
        }
    }
    match split_index {
        Some(i) => (
            &text[..i],
            text.get(i + 1..).or_else(|| text.get(i..)).unwrap(),
        ),
        _ => ("", text),
    }
}

#[derive(serde::Serialize)]
struct JsEncodeResult<'a> {
    quran: String,
    locations: Vec<JsLocation>,
    explanations: Vec<&'a str>,
}

#[derive(serde::Serialize)]
struct JsLocation {
    sura_number: u8,
    aya_number: u16,
    before_text: &'static str,
    text: &'static str,
    after_text: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let q = JsQuranize::new(0);
        let l = &q.encode("bismillah")[0].locations[0];
        assert_eq!(l.before_text, "");
        assert_eq!(l.text, "بِسْمِ اللَّهِ");
        assert_eq!(l.after_text, "الرَّحْمَـٰنِ الرَّحِيمِ");
    }

    #[test]
    fn test_split_at_nth_word() {
        assert_eq!(split_at_nth_word("ab cde fg h", 2), ("ab cde", "fg h"));
        assert_eq!(split_at_nth_word("fg h", 1), ("fg", "h"));
        assert_eq!(split_at_nth_word("ab c", 0), ("", "ab c"));
        assert_eq!(split_at_nth_word("ab c", 2), ("ab c", ""));
    }
}
