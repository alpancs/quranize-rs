use quranize::Quranize;
use serde_wasm_bindgen::{Error, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = Quranize)]
#[derive(Default)]
pub struct JsQuranize {
    quranize: Quranize,
}

#[derive(serde::Serialize)]
struct JsEncodeResult {
    quran: String,
    location_count: usize,
    explanations: Vec<JsExplanation>,
}

#[derive(serde::Serialize)]
struct JsExplanation {
    alphabet: String,
    quran: String,
}

#[derive(serde::Serialize)]
struct JsLocation<'a> {
    page: u16,
    sura: u8,
    aya: u16,
    before_text: &'a str,
    text: &'a str,
    after_text: &'a str,
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
                explanations: self.compress_explanation(&quran, explanations),
                quran,
                location_count,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = getLocations)]
    pub fn js_get_locations(&self, query: &str) -> Result<JsValue, Error> {
        to_value(&self.get_locations(query))
    }

    fn get_locations(&self, query: &str) -> Vec<JsLocation<'_>> {
        let mut indexes = self.quranize.find(query);
        indexes.sort_unstable();
        indexes
            .into_iter()
            .map(|(i, j)| {
                let (p, s, a, q) = self.quranize.get_data(i).copied().unwrap_or_default();
                let offset = q
                    .get(j + query.len()..)
                    .and_then(|s| Some(s.split(' ').next()?.len()))
                    .unwrap_or_default();
                let k = j + query.len() + offset;
                JsLocation {
                    page: p,
                    sura: s,
                    aya: a,
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

    fn get_page(&self, page: u16) -> Vec<JsQuranPageData<'_>> {
        self.quranize
            .get_data_from_page(page)
            .unwrap_or_default()
            .into_iter()
            .map(|&(_, sura, aya, text)| JsQuranPageData { sura, aya, text })
            .collect()
    }

    fn compress_explanation(&self, quran: &str, explanations: Vec<&str>) -> Vec<JsExplanation> {
        { quran.chars().zip(explanations) }
            .fold(Vec::new(), |mut aqs: Vec<JsExplanation>, (q, e)| {
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
        let l = locs.iter().find(|l| l.sura == 1).unwrap();
        assert_eq!(1, l.sura);
        assert_eq!(1, l.aya);
        assert_eq!("", l.before_text);
        assert_eq!("بِسمِ اللَّهِ", l.text);
        assert_eq!(" الرَّحمـٰنِ الرَّحيمِ", l.after_text);

        let l = &q.get_locations(&q.encode("bismillahirrohmanirrohim")[0].quran)[0];
        assert_eq!("", l.before_text);
        assert_eq!("بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيمِ", l.text);
        assert_eq!("", l.after_text);

        let l = &q.get_locations(&q.encode("arrohmanirrohim")[0].quran)[0];
        assert_eq!(1, l.sura);
        assert_eq!(1, l.aya);
        assert_eq!("بِسمِ اللَّهِ ", l.before_text);
        assert_eq!("الرَّحمـٰنِ الرَّحيمِ", l.text);
        assert_eq!("", l.after_text);
    }

    #[test]
    fn test_compress_explanation() {
        let q = JsQuranize::new();
        let quran = "بِرَبِّ النّاسِ";
        let expl = "b-i-r-o-b-b-i----n-n-a-s-".split('-').collect();
        assert_eq!(
            q.compress_explanation(quran, expl)
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
