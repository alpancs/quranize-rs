mod quran;
mod quran_index;
mod transliteration_map;

use quran_index::Node;

type EncodeResults<'a> = Vec<(String, &'a [(u8, u16, u8)])>;

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct Quranize {
    root: Node,
    transliteration_map: transliteration_map::Map,
    aya_index: quran_index::AyaMap,
}

impl Default for Quranize {
    fn default() -> Self {
        Self::new(u8::MAX)
    }
}

impl Quranize {
    pub fn new(word_count_limit: u8) -> Self {
        Self {
            root: quran_index::build_quran_index(word_count_limit),
            transliteration_map: transliteration_map::build_map(),
            aya_index: quran_index::build_aya_map(),
        }
    }

    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(&self.root, &normalize(text));
        results.dedup_by(|(q1, _), (q2, _)| q1 == q2);
        results
            .into_iter()
            .map(|(q, ls)| (q.chars().rev().collect(), ls))
            .collect()
    }

    fn rev_encode<'a>(&'a self, node: &'a Node, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !node.locations.is_empty() {
            results.push((String::new(), &node.locations));
        }
        for subnode in node.next_harfs.iter() {
            for prefix in self.transliteration_map[&subnode.content].iter() {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_subnode(subnode, subtext));
                }
            }
            if node.content == ' ' && subnode.content == 'ا' {
                results.append(&mut self.rev_encode_subnode(subnode, text));
            }
            if node.content == 'ا' && subnode.content == 'ل' {
                results.append(&mut self.rev_encode_subnode(subnode, text));
            }
            if node.content == 'و' && subnode.content == 'ا' {
                results.append(&mut self.rev_encode_subnode(subnode, text));
            }
        }
        results
    }

    fn rev_encode_subnode<'a>(&'a self, subnode: &'a Node, subtext: &str) -> EncodeResults {
        let mut results = self.rev_encode(subnode, subtext);
        let content = subnode.content;
        results.iter_mut().for_each(|(q, _)| q.push(content));
        results
    }

    pub fn get_aya(&self, sura_number: u8, aya_number: u16) -> &str {
        self.aya_index.get(&(sura_number, aya_number)).unwrap()
    }
}

fn normalize(text: &str) -> String {
    let mut text: Vec<_> = text
        .chars()
        .filter_map(|c| match c.to_ascii_lowercase() {
            c @ ('a'..='z' | '\'') => Some(c),
            _ => None,
        })
        .collect();
    text.dedup_by(|a, b| a == b && *a != 'l' && *a != 'a' && *a != 'o' && *a != 'i' && *a != 'u');
    String::from_iter(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quranize_short() {
        let q = Quranize::new(3);
        assert_eq!(encode(&q, "bismillah"), vec!["بسم الله", "بشماله"]);
        assert_eq!(encode(&q, "birobbinnas"), vec!["برب الناس"]);
        assert_eq!(encode(&q, "inna anzalnahu"), vec!["إنا أنزلناه"]);
        assert_eq!(encode(&q, "wabarro"), vec!["وبئر", "وبرا"]);
        assert_eq!(encode(&q, "idza qodho"), vec!["إذا قضى"]);
        assert_eq!(encode(&q, "masyaallah"), vec!["ما شاء الله"]);
        assert_eq!(encode(&q, "illa man taaba"), vec!["إلا من تاب"]);
        assert_eq!(encode(&q, "alla tahzani"), vec!["ألا تحزني"]);
        assert_eq!(encode(&q, "innasya niaka"), vec!["إن شانئك"]);
        assert_eq!(encode(&q, "wasalamun alaihi"), vec!["وسلام عليه"]);
    }

    #[test]
    fn test_quranize_full() {
        let q = Quranize::default();
        assert_eq!(
            encode(&q, "bismilla hirrohman nirrohiim"),
            vec!["بسم الله الرحمن الرحيم"]
        );
        assert_eq!(
            encode(&q, "alhamdulilla hirobbil 'alamiin"),
            vec!["الحمد لله رب العالمين"]
        );
        assert_eq!(
            encode(&q, "wa'tasimu bihablillah"),
            vec!["واعتصموا بحبل الله"]
        );
    }

    fn encode(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|(q, _)| q).collect()
    }

    #[test]
    fn test_quranize_empty_result() {
        let q = Quranize::new(2);
        assert!(q.encode("").is_empty());
        assert!(q.encode("aaa").is_empty());
        assert!(q.encode("bbb").is_empty());
        assert!(q.encode("abcd").is_empty());
        assert!(q.encode("1+2=3").is_empty());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(""), "");
        assert_eq!(normalize("bismi"), "bismi");
        assert_eq!(normalize("'aalimul ghoibi"), "'aalimulghoibi");
        assert_eq!(normalize("Qul A'udzu"), "qula'udzu");
        assert_eq!(
            normalize("bismilla hirrohman nirrohiim"),
            "bismillahirohmanirohiim"
        );
    }
}
