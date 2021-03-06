mod quran_index;
mod transliteration_map;
mod word_utils;

use quran_index::Node;
use transliteration_map::transliterations;

type EncodeResults<'a> = Vec<(String, &'a [(u8, u16, u8)], Vec<&'a str>)>;

pub struct Quranize {
    root: Node,
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
        }
    }

    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut results = self.rev_encode(&self.root, &normalize(text));
        results.dedup_by(|r1, r2| r1.0 == r2.0);
        results
            .into_iter()
            .map(|(q, l, e)| (q.chars().rev().collect(), l, e.into_iter().rev().collect()))
            .collect()
    }

    fn rev_encode<'a>(&'a self, node: &'a Node, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !node.locations.is_empty() {
            results.push((String::new(), &node.locations, Vec::new()));
        }
        for subnode in node.next_harfs.iter() {
            for prefix in transliterations(subnode.content).iter().rev() {
                if let Some(subtext) = text.strip_prefix(prefix) {
                    results.append(&mut self.rev_encode_sub(subnode, subtext, prefix));
                }
            }
            if is_special_case(node.content, subnode.content) {
                results.append(&mut self.rev_encode_sub(subnode, text, ""));
            }
        }
        results
    }

    fn rev_encode_sub<'a>(&'a self, node: &'a Node, text: &str, expl: &'a str) -> EncodeResults {
        let mut results = self.rev_encode(node, text);
        for (q, _, e) in results.iter_mut() {
            q.push(node.content);
            e.push(expl);
        }
        results
    }
}

fn normalize(text: &str) -> String {
    let mut chars = Vec::from_iter(text.chars().filter_map(|c| match c.to_ascii_lowercase() {
        c @ ('a'..='z' | '\'') => Some(c),
        _ => None,
    }));
    chars.dedup_by(|a, b| a == b && *a != 'l' && *a != 'a' && *a != 'o' && *a != 'i' && *a != 'u');
    String::from_iter(chars)
}

fn is_special_case(node_content: char, subnode_content: char) -> bool {
    (node_content == ' ' && subnode_content == '??')
        || (node_content == '??' && subnode_content == '??')
        || (node_content == '??' && subnode_content == '??')
        || (node_content == '??' && subnode_content == '??')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quranize_short() {
        let q = Quranize::new(3);
        assert_eq!(encode(&q, "bismillah"), vec!["?????? ????????", "????????????"]);
        assert_eq!(encode(&q, "birobbinnas"), vec!["?????? ??????????"]);
        assert_eq!(encode(&q, "inna anzalnahu"), vec!["?????? ??????????????"]);
        assert_eq!(encode(&q, "wa'tasimu"), vec!["????????????????"]);
        assert_eq!(encode(&q, "wabarro"), vec!["????????", "????????"]);
        assert_eq!(encode(&q, "idza qodho"), vec!["?????? ??????"]);
        assert_eq!(encode(&q, "masyaallah"), vec!["???? ?????? ????????"]);
        assert_eq!(encode(&q, "illa man taaba"), vec!["?????? ???? ??????"]);
        assert_eq!(encode(&q, "qulhuwallahuahad"), Vec::<String>::new());
        assert_eq!(encode(&q, "alla tahzani"), vec!["?????? ??????????"]);
        assert_eq!(encode(&q, "innasya niaka"), vec!["???? ??????????"]);
        assert_eq!(encode(&q, "wasalamun alaihi"), vec!["?????????? ????????"]);
        assert_eq!(encode(&q, "ulaika hum"), vec!["?????????? ????"]);
        assert_eq!(encode(&q, "waladdoolin"), vec!["?????? ??????????????"]);
        assert_eq!(encode(&q, "n"), vec!["??"]);
    }

    #[test]
    fn test_quranize_full() {
        let q = Quranize::default();
        assert_eq!(
            encode(&q, "bismilla hirrohman nirrohiim"),
            vec!["?????? ???????? ???????????? ????????????"]
        );
        assert_eq!(
            encode(&q, "alhamdulilla hirobbil 'alamiin"),
            vec!["?????????? ?????? ???? ????????????????"]
        );
        assert_eq!(encode(&q, "arrohma nirrohim"), vec!["???????????? ????????????"]);
        assert_eq!(encode(&q, "maliki yau middin"), vec!["???????? ?????? ??????????"]);
        assert_eq!(
            encode(&q, "iyyakanakbudu waiyyakanastain"),
            vec!["???????? ???????? ?????????? ????????????"]
        );
        assert_eq!(
            encode(&q, "ihdinassirotol mustaqim"),
            vec!["?????????? ???????????? ????????????????"]
        );
        assert_eq!(
            encode(
                &q,
                "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"
            ),
            vec!["???????? ?????????? ?????????? ?????????? ?????? ?????????????? ?????????? ?????? ??????????????"]
        );
        assert_eq!(encode(&q, "qulhuwallahuahad"), vec!["???? ???? ???????? ??????"]);
    }

    fn encode(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|r| r.0).collect()
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(3);
        assert_eq!(q.encode("bismillah")[0].1[0], (1, 1, 1));
        assert_eq!(q.encode("subhanallah")[0].1.len(), 5);
        assert_eq!(q.encode("arrohman").len(), 1);
        assert_eq!(q.encode("arrohman")[0].1.len(), 45);
        assert_eq!(q.encode("alhamdu")[0].2, vec!["a", "l", "ha", "m", "du"]);
        assert_eq!(
            q.encode("arrohman")[0].2,
            vec!["a", "", "ro", "h", "ma", "n"]
        );
        let result = &q.encode("masyaallah")[0];
        assert_eq!(result.0.chars().count(), result.2.len());
        assert_eq!(
            result.2,
            vec!["m", "a", "", "sy", "a", "a", "", "", "l", "la", "h"]
        );
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
