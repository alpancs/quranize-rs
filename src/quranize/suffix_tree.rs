use std::collections::{btree_set::Range, BTreeSet};

use super::word_suffixes::WordSuffixIterExt;

type Data = (u16, u16);
type Vertex = Option<Data>;
pub(super) type Edge<'a> = (usize, usize, &'a str);

pub(super) struct SuffixTree<'a> {
    vertices: Vec<Vertex>,
    edges: BTreeSet<Edge<'a>>,
}

impl<'a> SuffixTree<'a> {
    pub(super) fn new() -> Self {
        Self {
            vertices: vec![None],
            edges: Default::default(),
        }
    }

    pub(super) fn construct(&mut self, line_offset: u16, s: &'a str) {
        s.word_suffixes()
            .for_each(|(i, suf)| self.construct_suffix((line_offset, i as u16), 0, suf));
    }

    fn construct_suffix(&mut self, d: Data, root: usize, subs: &'a str) {
        let mergeable_edge =
            { self.v_edges(root) }.find_map(|e| Self::longest_prefix(subs, e.2).map(|p| (*e, p)));
        match mergeable_edge {
            Some((e, p)) if e.2 == p => self.construct_suffix(d, e.1, &subs[p.len()..]),
            Some((e, p)) => {
                let v = self.add_vertex(None);
                self.edges.remove(&e);
                self.edges.insert((e.0, v, p));
                self.edges.insert((v, e.1, &e.2[p.len()..]));
                self.construct_suffix(d, v, &subs[p.len()..])
            }
            None => {
                let v = self.add_vertex(Some(d));
                self.edges.insert((root, v, subs));
            }
        }
    }

    pub(super) fn v_edges(&self, v: usize) -> Range<Edge<'a>> {
        self.edges.range((v, 0, "")..(v + 1, 0, ""))
    }

    fn longest_prefix(s: &'a str, t: &str) -> Option<&'a str> {
        let pairs = s.char_indices().zip(t.char_indices());
        let last_pair = pairs.take_while(|((_, cs), (_, ct))| cs == ct).last()?;
        Some(&s[..last_pair.0 .0 + last_pair.0 .1.len_utf8()])
    }

    fn add_vertex(&mut self, v: Vertex) -> usize {
        self.vertices.push(v);
        self.vertices.len() - 1
    }
}

#[cfg(test)]
impl SuffixTree<'_> {
    pub(super) fn to_mermaid(&self) -> String {
        std::iter::once("graph TB\n".to_string())
            .chain(self.edges.iter().enumerate().map(|(i, e)| {
                format!(
                    "  v{}(({})) -- \"{}\" --> v{}(({}))\n",
                    e.0,
                    self.data(e.0),
                    format!("E<sub>{}</sub>: ", i)
                        + match e.2 {
                            "" => "&nbsp;",
                            "#" => "&nbsp;#&nbsp;",
                            _ => e.2,
                        },
                    e.1,
                    self.data(e.1)
                )
            }))
            .collect()
    }

    fn data(&self, v: usize) -> String {
        let content = self.vertices[v]
            .map(|d| format!("({}, {})", d.0, d.1))
            .unwrap_or("&nbsp;".repeat(3));
        format!("\"[{}]<br>{}\"", v, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suffix_tree_for_quran() {
        let mut t = SuffixTree::new();
        for (id, s) in (0..3).zip(include_str!("../quran-uthmani-min.txt").split('\n')) {
            t.construct(id, s);
        }
        println!("{}", t.to_mermaid());
        assert_eq!(t.edges.len(), 116_426);
        assert_eq!(t.vertices.len() - t.edges.len(), 1);
    }

    #[test]
    fn test_longest_prefix() {
        assert_eq!(SuffixTree::longest_prefix("", ""), None);
        assert_eq!(SuffixTree::longest_prefix("x", ""), None);
        assert_eq!(SuffixTree::longest_prefix("", "y"), None);
        assert_eq!(SuffixTree::longest_prefix("x", "y"), None);
        assert_eq!(SuffixTree::longest_prefix("a", "a"), Some("a"));
        assert_eq!(SuffixTree::longest_prefix("ax", "a"), Some("a"));
        assert_eq!(SuffixTree::longest_prefix("a", "ay"), Some("a"));
        assert_eq!(SuffixTree::longest_prefix("ax", "ay"), Some("a"));
    }
}
