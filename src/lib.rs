#![feature(test)]

mod quran_index;
use quran_index::{build_quran_index, Harf};

mod quranize_map;
use quranize_map::{build_quranize_map, QuranizeMap};

pub struct Quranize {
    quran_index: Harf,
    quranize_map: QuranizeMap,
}

pub fn build_quranize() -> Quranize {
    Quranize {
        quran_index: build_quran_index(),
        quranize_map: build_quranize_map(),
    }
}

type EncodeResult = Vec<(String, Vec<(u8, u16, u8)>)>;

impl Quranize {
    pub fn encode(&self, text: &str) -> EncodeResult {
        self.encode_with_context(text, &self.quran_index)
    }

    fn encode_with_context(&self, text: &str, context: &Harf) -> EncodeResult {
        let mut results = Vec::new();
        for (alphabet, qurans) in self.quranize_map.iter() {
            if !text.starts_with(alphabet) {
                continue;
            }
            for quran in qurans {
                for harf in context.next_harfs.iter() {
                    if *quran == harf.content {
                        let subtext = &text[alphabet.len()..];
                        if subtext.is_empty() {
                            if !harf.locations.is_empty() {
                                results.push((quran.to_string(), harf.locations.to_vec()));
                            }
                        } else {
                            let subresults = self.encode_with_context(subtext, harf);
                            results.append(
                                &mut subresults
                                    .into_iter()
                                    .map(|(q, l)| (quran.to_string() + &q, l))
                                    .collect(),
                            );
                        }
                        break;
                    }
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quranize() {
        let quranize = build_quranize();
        assert_eq!(
            quranize
                .encode("bismi")
                .iter()
                .map(|(q, _)| q)
                .collect::<Vec<_>>(),
            "بإثمي بئسما باسم بعصم بسم"
                .split_whitespace()
                .collect::<Vec<_>>(),
        );
        assert_eq!(quranize.encode("").len(), 0);
        assert_eq!(quranize.encode("mxkasmxka").len(), 0);
    }
}
