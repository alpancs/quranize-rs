/*!
Transforms transliteration back into Quran form.

# Examples

```
let q = quranize::Quranize::default();
assert_eq!(q.encode("alhamdulillah").first().unwrap().0, "الحمد لله");
```

# Crate features

In addition to [`SIMPLE_CLEAN`][quran::SIMPLE_CLEAN], the [`quran`] module also has [`SIMPLE_PLAIN`][quran::SIMPLE_PLAIN].
It can be used by enabling feature `quran-simple-plain`.
The feature is not enabled by default to keep the [`quran`] module as small as possible.
To enable the feature, add the following lines to `Cargo.toml` file.

```toml
[dependencies]
quranize = { version = "0.4", features = ["quran-simple-plain"] }
```
*/

mod normalization;
pub mod quran;
mod quran_index;
mod transliterations;

use quran_index::Node;

type EncodeResults<'a> = Vec<(String, Vec<&'a str>)>;

/// Struct to encode transliterations into Quran forms.
pub struct Quranize {
    root: Node,
}

impl Default for Quranize {
    /// Build [`Quranize`] without [word count limit][Quranize::new].
    ///
    /// # Examples
    /// ```
    /// let q = quranize::Quranize::default();
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاء الله");
    /// ```
    fn default() -> Self {
        Self::new(u8::MAX)
    }
}

impl Quranize {
    /// Build [`Quranize`] with parameter `word_count_limit`.
    /// It limits the number of consecutive words scanned by the indexer to reduce memory usage and indexing time.
    /// Use [`Quranize::default`] to build [`Quranize`] without the limit.
    ///
    /// # Examples
    /// ```
    /// let q = quranize::Quranize::new(5);
    /// assert_eq!(q.encode("masyaallah").first().unwrap().0, "ما شاء الله");
    /// let q = quranize::Quranize::new(1);
    /// assert_eq!(q.encode("masyaallah").first(), None);
    /// ```
    pub fn new(word_count_limit: u8) -> Self {
        Self {
            root: quran_index::build_quran_index(word_count_limit),
        }
    }

    /// Encode `text` back into Quran form.
    pub fn encode(&self, text: &str) -> EncodeResults {
        let mut rev_results = self.rev_encode(&self.root, &normalization::normalize(text));
        rev_results.dedup_by(|r1, r2| r1.0 == r2.0);
        rev_results
            .into_iter()
            .map(|(q, e)| (q.chars().rev().collect(), e.into_iter().rev().collect()))
            .rev()
            .collect()
    }

    fn rev_encode<'a>(&'a self, node: &'a Node, text: &str) -> EncodeResults {
        let mut results = EncodeResults::new();
        if text.is_empty() && !node.locations.is_empty() {
            results.push((String::new(), Vec::new()));
        }
        for subnode in node.next_harfs.iter() {
            for prefix in transliterations::map(subnode.content).iter().rev() {
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
        for (q, e) in results.iter_mut() {
            q.push(node.content);
            e.push(expl);
        }
        results
    }
}

fn is_special_case(node_content: char, subnode_content: char) -> bool {
    (node_content == ' ' && subnode_content == 'ا')
        || (node_content == 'ا' && subnode_content == 'ل')
        || (node_content == 'و' && subnode_content == 'ا')
        || (node_content == 'أ' && subnode_content == 'و')
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
        assert_eq!(encode(&q, "wa'tasimu"), vec!["واعتصموا"]);
        assert_eq!(encode(&q, "wabarro"), vec!["وبئر", "وبرا"]);
        assert_eq!(encode(&q, "idza qodho"), vec!["إذا قضى"]);
        assert_eq!(encode(&q, "masyaallah"), vec!["ما شاء الله"]);
        assert_eq!(encode(&q, "illa man taaba"), vec!["إلا من تاب"]);
        assert_eq!(encode(&q, "qulhuwallahuahad"), Vec::<String>::new());
        assert_eq!(encode(&q, "alla tahzani"), vec!["ألا تحزني"]);
        assert_eq!(encode(&q, "innasya niaka"), vec!["إن شانئك"]);
        assert_eq!(encode(&q, "wasalamun alaihi"), vec!["وسلام عليه"]);
        assert_eq!(encode(&q, "ulaika hum"), vec!["أولئك هم"]);
        assert_eq!(encode(&q, "waladdoolin"), vec!["ولا الضالين"]);
        assert_eq!(encode(&q, "n"), vec!["ن"]);
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
        assert_eq!(encode(&q, "arrohma nirrohim"), vec!["الرحمن الرحيم"]);
        assert_eq!(encode(&q, "maliki yau middin"), vec!["مالك يوم الدين"]);
        assert_eq!(
            encode(&q, "iyyakanakbudu waiyyakanastain"),
            vec!["إياك نعبد وإياك نستعين"]
        );
        assert_eq!(
            encode(&q, "ihdinassirotol mustaqim"),
            vec!["اهدنا الصراط المستقيم"]
        );
        assert_eq!(
            encode(
                &q,
                "shirotolladzina an'amta 'alaihim ghoiril maghdzubi 'alaihim waladdoolliin"
            ),
            vec!["صراط الذين أنعمت عليهم غير المغضوب عليهم ولا الضالين"]
        );
        assert_eq!(encode(&q, "qulhuwallahuahad"), vec!["قل هو الله أحد"]);
    }

    fn encode(quranize: &Quranize, text: &str) -> Vec<String> {
        quranize.encode(text).into_iter().map(|r| r.0).collect()
    }

    #[test]
    fn test_quranize_misc() {
        let q = Quranize::new(3);
        assert_eq!(q.encode("bismillah")[0].1.len(), 8);
        assert_eq!(q.encode("arrohman").len(), 1);
        assert_eq!(q.encode("arrohman")[0].1.len(), 6);
        assert_eq!(q.encode("alhamdu")[0].1, vec!["a", "l", "ha", "m", "du"]);
        assert_eq!(
            q.encode("arrohman")[0].1,
            vec!["a", "", "ro", "h", "ma", "n"]
        );
        let result = &q.encode("masyaallah")[0];
        assert_eq!(result.0.chars().count(), result.1.len());
        assert_eq!(
            result.1,
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
}
