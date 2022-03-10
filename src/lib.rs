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
        if text.is_empty() {
            return if context.locations.is_empty() {
                vec![]
            } else {
                vec![("".to_string(), context.locations.to_vec())]
            };
        }

        let mut results = vec![];
        for next_harf in context.next_harfs.iter() {
            let content = next_harf.content;
            for transliteration in self.transliteration_map[&content].iter() {
                if text.starts_with(transliteration) {
                    let subtext = &text[transliteration.len()..];
                    let subresults = self.encode_with_context(subtext, next_harf);
                    results.append(
                        &mut subresults
                            .into_iter()
                            .map(|(q, l)| (content.to_string() + &q, l))
                            .collect(),
                    );
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_quranize_bismi() {
        let quranize = build_quranize();
        assert_eq!(
            quranize
                .encode("bismi")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            vec!["باسم", "بعصم", "بئسما", "بإثمي", "بسم"],
        );
    }

    #[test]
    fn test_quranize_bismillah() {
        let quranize = build_quranize();
        assert_eq!(
            quranize
                .encode("bismillah")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            vec!["بسم الله"],
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
        b.iter(|| quranize.encode("bismi"));
    }
}
