#![feature(test)]

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod quranize_map;
use quranize_map::{build_transliteration_map, TransliterationMap};

pub struct Quranize {
    quran_index: Harf,
    transliteration_map: TransliterationMap,
}

pub fn build_quranize() -> Quranize {
    Quranize {
        quran_index: build_quran_index(),
        transliteration_map: build_transliteration_map(),
    }
}

type EncodeResult = Vec<(String, Vec<(u8, u16, u8)>)>;

impl Quranize {
    pub fn encode(&self, text: &str) -> EncodeResult {
        self.encode_with_context(text, &self.quran_index)
    }

    fn encode_with_context(&self, text: &str, context: &Harf) -> EncodeResult {
        match (text, &context.locations) {
            ("", locations) if locations.is_empty() => vec![],
            ("", locations) => vec![("".to_string(), locations.to_vec())],
            _ => {
                let mut results = vec![];
                for next_harf in context.next_harfs.iter() {
                    let content = next_harf.content;
                    for trs in self.transliteration_map[&content].iter() {
                        if text.starts_with(trs) {
                            let subresults = self
                                .encode_with_context(&text[trs.len()..], next_harf)
                                .into_iter()
                                .map(|(q, l)| (content.to_string() + &q, l));
                            results.append(&mut subresults.collect());
                        }
                    }
                }
                results
            }
        }
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
            quranize
                .encode("bismi")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            vec!["باسم", "بعصم", "بئسما", "بإثمي", "بسم"],
        );
        assert_eq!(
            quranize
                .encode("bismillah")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            vec!["بسم الله"],
        );
        assert_eq!(
            quranize.encode("bismillah").first().unwrap().1,
            vec![(1, 1, 1), (11, 41, 4), (27, 30, 5)],
        );
        assert_eq!(
            quranize
                .encode("bisyimaalihi")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            vec!["بشماله"],
        );
    }

    #[test]
    fn test_quranize_zero() {
        let quranize = build_quranize();
        assert!(quranize.encode("").is_empty());
        assert!(quranize.encode("gsquw").is_empty());
    }

    #[bench]
    fn bench_quranize(b: &mut Bencher) {
        let quranize = build_quranize();
        b.iter(|| quranize.encode("bismillah"));
    }
}
