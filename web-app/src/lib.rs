use quranize::Quranize;
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Quranize)]
pub struct JsQuranize {
    quranize: Quranize,
}

#[derive(serde::Serialize)]
struct JsEncodeResult {
    quran: String,
    explanation: String,
    location_count: usize,
}

#[derive(serde::Serialize)]
struct JsLocation<'a> {
    index: usize,
    sura_number: u8,
    aya_number: u16,
    before_text: &'a str,
    text: &'a str,
    after_text: &'a str,
}

#[derive(serde::Serialize)]
struct JsExplanation {
    alphabet: String,
    quran: String,
}

#[wasm_bindgen(js_class = Quranize)]
impl JsQuranize {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let quranize = Quranize::new();
        Self { quranize }
    }

    #[wasm_bindgen(js_name = encode)]
    pub fn js_encode(&self, text: &str) -> Result<JsValue, Error> {
        to_value(&self.encode(text))
    }

    fn encode(&self, text: &str) -> Vec<JsEncodeResult> {
        self.quranize
            .encode(text)
            .into_iter()
            .map(|(quran, location_count, explanations)| JsEncodeResult {
                quran,
                explanation: explanations.join("-"),
                location_count,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = getLocations)]
    pub fn js_get_locations(&self, quran: &str) -> Result<JsValue, Error> {
        to_value(&self.get_locations(quran))
    }

    fn get_locations(&self, quran: &str) -> Vec<JsLocation> {
        { self.quranize.find(quran).into_iter() }
            .map(|(i, j)| {
                let sura_number = self.quranize.get_sura(i).unwrap_or_default();
                let aya_number = self.quranize.get_aya(i).unwrap_or_default();
                let aya = self.quranize.get_quran(i).unwrap_or_default();
                let offset = aya
                    .get(j + quran.len()..)
                    .and_then(|s| Some(s.split(' ').next()?.len()))
                    .unwrap_or_default();
                let k = j + quran.len() + offset;
                JsLocation {
                    index: i,
                    sura_number,
                    aya_number,
                    before_text: aya.get(..j).unwrap_or_default(),
                    text: aya.get(j..k).unwrap_or_default(),
                    after_text: aya.get(k..).unwrap_or_default(),
                }
            })
            .collect()
    }
}

impl Default for JsQuranize {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_name = compressExplanation)]
pub fn js_compress_explanation(quran: &str, explanation: &str) -> Result<JsValue, Error> {
    to_value(&compress_explanation(quran, explanation))
}

fn compress_explanation(quran: &str, explanation: &str) -> Vec<JsExplanation> {
    { explanation.split('-').zip(quran.chars()) }
        .fold(Vec::new(), |mut aqs: Vec<JsExplanation>, (e, q)| {
            match (aqs.last_mut(), q) {
                (Some(laq), '\u{064B}'..='\u{0651}' | '\u{0670}') => {
                    laq.alphabet.push_str(e);
                    laq.quran.push(q);
                }
                _ => aqs.push(JsExplanation {
                    alphabet: e.to_string(),
                    quran: q.to_string(),
                }),
            }
            aqs
        })
        .into_iter()
        .fold(Vec::new(), |mut aqs, eq| {
            match aqs.last_mut() {
                Some(laq) if laq.alphabet.is_empty() => {
                    laq.alphabet += &eq.alphabet;
                    laq.quran += &eq.quran;
                }
                _ => aqs.push(eq),
            }
            aqs
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_encode() {
        let q = JsQuranize::new();
        let locs = &q.get_locations(&q.encode("bismillah")[0].quran);
        let l = locs.iter().find(|l| l.sura_number == 1).unwrap();
        assert_eq!(1, l.sura_number);
        assert_eq!(1, l.aya_number);
        assert_eq!("", l.before_text);
        assert_eq!("بِسمِ اللَّهِ", l.text);
        assert_eq!(" الرَّحمـٰنِ الرَّحيمِ", l.after_text);

        let l = &q.get_locations(&q.encode("bismillahirrohmanirrohim")[0].quran)[0];
        assert_eq!("", l.before_text);
        assert_eq!("بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيمِ", l.text);
        assert_eq!("", l.after_text);

        let l = &q.get_locations(&q.encode("arrohmanirrohim")[0].quran)[0];
        assert_eq!(1, l.sura_number);
        assert_eq!(1, l.aya_number);
        assert_eq!("بِسمِ اللَّهِ ", l.before_text);
        assert_eq!("الرَّحمـٰنِ الرَّحيمِ", l.text);
        assert_eq!("", l.after_text);
    }

    #[test]
    fn test_compress_explanation() {
        assert_eq!(
            compress_explanation("بِرَبِّ النّاسِ", "b-i-r-o-b-b-i----n-n-a-s-")
                .into_iter()
                .map(|aq| (aq.alphabet, aq.quran))
                .collect::<Vec<_>>(),
            [
                ("bi", "بِ"),
                ("ro", "رَ"),
                ("bbi", "بِّ"),
                ("nn", " النّ"),
                ("a", "ا"),
                ("s", "سِ"),
            ]
            .map(|(x, y)| (x.to_string(), y.to_string()))
        );
    }
}
