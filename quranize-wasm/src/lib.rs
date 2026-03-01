use std::iter::once;

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
    spans: Vec<JsLocationSpan<'a>>,
}

#[derive(serde::Serialize)]
struct JsLocationSpan<'a> {
    text: &'a str,
    marked: bool,
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
            .chunk_by(|(i, _), (next_i, _)| i == next_i)
            .map(|ijs| {
                let (i, first_j) = ijs.first().copied().unwrap_or_default();
                let (p, s, a, q) = self.quranize.get_data(i).copied().unwrap_or_default();
                let first_span = JsLocationSpan {
                    text: q.get(..first_j).unwrap_or_default(),
                    marked: false,
                };
                let js = ijs.iter().map(|&(_, j)| j);
                let next_js = ijs.iter().skip(1).map(|&(_, j)| j).chain(once(q.len()));
                let spans = js.zip(next_js).flat_map(|(j, next_j)| {
                    let marked_len = query.len()
                        + q.get(j + query.len()..)
                            .and_then(|s| Some(s.split(' ').next()?.len()))
                            .unwrap_or_default();
                    [
                        JsLocationSpan {
                            text: q.get(j..j + marked_len).unwrap_or_default(),
                            marked: true,
                        },
                        JsLocationSpan {
                            text: q.get(j + marked_len..next_j).unwrap_or_default(),
                            marked: false,
                        },
                    ]
                });
                JsLocation {
                    page: p,
                    sura: s,
                    aya: a,
                    spans: once(first_span).chain(spans).collect(),
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
        assert_eq!("", l.spans[0].text);
        assert_eq!("بِسمِ اللَّهِ", l.spans[1].text);
        assert_eq!(" الرَّحمـٰنِ الرَّحيمِ", l.spans[2].text);

        let l = &q.get_locations(&q.encode("bismillahirrohmanirrohim")[0].quran)[0];
        assert_eq!("", l.spans[0].text);
        assert_eq!("بِسمِ اللَّهِ الرَّحمـٰنِ الرَّحيمِ", l.spans[1].text);
        assert_eq!("", l.spans[2].text);

        let l = &q.get_locations(&q.encode("arrohmanirrohim")[0].quran)[0];
        assert_eq!(1, l.sura);
        assert_eq!(1, l.aya);
        assert_eq!("بِسمِ اللَّهِ ", l.spans[0].text);
        assert_eq!("الرَّحمـٰنِ الرَّحيمِ", l.spans[1].text);
        assert_eq!("", l.spans[2].text);

        let albaqara_183 = &q.get_locations(&q.encode("kutiba ala")[0].quran)[2];
        assert_eq!(2, albaqara_183.sura);
        assert_eq!(183, albaqara_183.aya);
        let mut spans = albaqara_183.spans.iter();
        assert_eq!("يا أَيُّهَا الَّذينَ آمَنوا ", spans.next().unwrap().text);
        assert_eq!("كُتِبَ عَلَيكُمُ", spans.next().unwrap().text);
        assert_eq!(" الصِّيامُ كَما ", spans.next().unwrap().text);
        assert_eq!("كُتِبَ عَلَى", spans.next().unwrap().text);
        assert_eq!(" الَّذينَ مِن قَبلِكُم لَعَلَّكُم تَتَّقونَ", spans.next().unwrap().text);
        assert!(spans.next().is_none());
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
