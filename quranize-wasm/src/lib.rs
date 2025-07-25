use quranize::Quranize;
use serde_wasm_bindgen::{to_value, Error};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Quranize)]
#[derive(Default)]
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
    page_number: u16,
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

#[derive(serde::Serialize)]
struct JsQuranPageData<'a> {
    sura: u8,
    aya: u16,
    text: &'a str,
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
    pub fn js_get_locations(&self, query: &str) -> Result<JsValue, Error> {
        to_value(&self.get_locations(query))
    }

    fn get_locations(&self, query: &str) -> Vec<JsLocation> {
        { self.quranize.find(query).into_iter() }
            .map(|(i, j)| {
                let (p, s, a, q) = self.quranize.get_data(i).copied().unwrap_or_default();
                let offset = q
                    .get(j + query.len()..)
                    .and_then(|s| Some(s.split(' ').next()?.len()))
                    .unwrap_or_default();
                let k = j + query.len() + offset;
                JsLocation {
                    page_number: p,
                    sura_number: s,
                    aya_number: a,
                    before_text: q.get(..j).unwrap_or_default(),
                    text: q.get(j..k).unwrap_or_default(),
                    after_text: q.get(k..).unwrap_or_default(),
                }
            })
            .collect()
    }

    #[wasm_bindgen(js_name = getPage)]
    pub fn js_get_page(&self, page: u16) -> Result<JsValue, Error> {
        to_value(&self.get_page(page))
    }

    fn get_page(&self, page: u16) -> Vec<JsQuranPageData> {
        self.quranize
            .get_data_from_page(page)
            .unwrap_or_default()
            .into_iter()
            .map(|&(_, sura, aya, text)| JsQuranPageData { sura, aya, text })
            .collect()
    }

    #[wasm_bindgen(js_name = compressExpl)]
    pub fn js_compress_explanation(&self, quran: &str, expl: &str) -> Result<JsValue, Error> {
        to_value(&self.compress_explanation(quran, expl))
    }

    fn compress_explanation(&self, quran: &str, explanation: &str) -> Vec<JsExplanation> {
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
        let q = JsQuranize::new();
        assert_eq!(
            q.compress_explanation("بِرَبِّ النّاسِ", "b-i-r-o-b-b-i----n-n-a-s-")
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
