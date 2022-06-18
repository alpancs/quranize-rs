use super::Quranize;
use wasm_bindgen::prelude::*;

#[allow(clippy::unused_unit)]
#[wasm_bindgen]
impl Quranize {
    #[wasm_bindgen(constructor)]
    pub fn js_new(word_count_limit: u8) -> Self {
        match word_count_limit {
            0 => Quranize::default(),
            n => Quranize::new(n),
        }
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn js_encode(&self, text: &str) -> JsValue {
        let encode_results = self
            .encode(text)
            .into_iter()
            .map(|(quran, locations, explanations)| {
                let word_count = quran.split_whitespace().count();
                JsEncodeResult {
                    quran,
                    locations: self.to_js_locations(locations, word_count),
                    explanations,
                }
            })
            .collect::<Vec<_>>();
        JsValue::from_serde(&encode_results).unwrap()
    }

    fn to_js_locations(&self, locations: &[(u8, u16, u8)], word_count: usize) -> Vec<JsLocation> {
        locations
            .iter()
            .map(|&(sura_number, aya_number, word_number)| {
                let w = word_number as usize;
                let aya_text = self.get_aya(sura_number, aya_number);
                JsLocation {
                    sura_number,
                    aya_number,
                    before_text: get_subword(aya_text, 0, w - 1),
                    text: get_subword(aya_text, w - 1, word_count),
                    after_text: get_subword(aya_text, w - 1 + word_count, usize::MAX),
                }
            })
            .collect()
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
    fn test_get_subword() {
        assert_eq!(&get_subword("ab cd ef gh", 1, 2), "cd ef");
        assert_eq!(&get_subword("ab cd ef gh", 2, 1), "ef");
        assert_eq!(&get_subword("ab cd ef gh", 2, usize::MAX), "ef gh");
    }
}
