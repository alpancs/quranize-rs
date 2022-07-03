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
                let aya_text = self.aya_map.get(sura_number, aya_number).unwrap();
                let mut words = aya_text.split_whitespace();
                JsLocation {
                    sura_number,
                    aya_number,
                    before_text: take_join(&mut words, (word_number - 1).into()),
                    text: take_join(&mut words, word_count),
                    after_text: take_join(&mut words, usize::MAX),
                }
            })
            .collect()
    }
}

fn take_join<'a>(words: &mut impl Iterator<Item = &'a str>, n_take: usize) -> String {
    Vec::from_iter(words.take(n_take)).join(" ")
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
    before_text: String,
    text: String,
    after_text: String,
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
    fn test_take_join() {
        let mut words = "ab cd ef g".split_whitespace();
        assert_eq!(&take_join(&mut words, 0), "");
        assert_eq!(&take_join(&mut words, 2), "ab cd");
        assert_eq!(&take_join(&mut words, usize::MAX), "ef g");
    }
}
