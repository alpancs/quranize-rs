use std::collections::{btree_set::Range, BTreeSet};

use super::word_suffixes::WordSuffixIterExt;

pub(super) type Data = (u16, u16);
pub(super) type Vertex = Option<Data>;
pub(super) type Edge<'a> = (usize, usize, &'a str);

pub(super) struct SuffixTree<'a> {
    vertices: Vec<Vertex>,
    edges: BTreeSet<Edge<'a>>,
}

impl<'a> SuffixTree<'a> {
    pub(super) const ROOT: usize = 0;

    pub(super) fn new() -> Self {
        Self {
            vertices: vec![None],
            edges: Default::default(),
        }
    }

    pub(super) fn construct(&mut self, i: usize, s: &'a str) {
        s.word_suffixes()
            .for_each(|(j, s)| self.construct_suffix((i as u16, j as u16), Self::ROOT, s));
    }

    fn construct_suffix(&mut self, d: Data, root: usize, subs: &'a str) {
        match { self.v_edges(root) }.find_map(|&e| Some(e).zip(Self::longest_prefix(subs, e.2))) {
            Some((e, p)) if e.2 == p => self.construct_suffix(d, e.1, &subs[p.len()..]),
            Some((e, p)) => {
                self.edges.remove(&e);
                let v = self.add_vertex(None);
                self.edges.insert((e.0, v, p));
                self.edges.insert((v, e.1, &e.2[p.len()..]));
                let w = self.add_vertex(Some(d));
                self.edges.insert((v, w, &subs[p.len()..]));
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

    pub(super) fn find_str(&self, s: &str, v: usize) -> Vec<Data> {
        self.v_edges(v)
            .filter(|&(_, _, l)| !s.is_empty() && !l.is_empty())
            .find_map(|&(_, w, l)| match (s.strip_prefix(l), l.strip_prefix(s)) {
                (_, Some(_)) => Some(self.collect_data(w)),
                (Some(s), _) => Some(self.find_str(s, w)),
                _ => None,
            })
            .unwrap_or_default()
    }

    fn collect_data(&self, v: usize) -> Vec<Data> {
        let head = std::iter::once(self.vertices[v]).flatten();
        let tail = self.v_edges(v).flat_map(|&(_, w, _)| self.collect_data(w));
        head.chain(tail).collect()
    }

    pub(super) fn count_data(&self, v: usize) -> usize {
        let parent_count: usize = self.vertices[v].is_some().into();
        let childs_count: usize = self.v_edges(v).map(|&(_, w, _)| self.count_data(w)).sum();
        parent_count + childs_count
    }
}

#[cfg(test)]
impl SuffixTree<'_> {
    pub(super) fn to_mermaid(&self) -> String {
        std::iter::once("graph TB\n".to_string())
            .chain(self.edges.iter().enumerate().map(|(i, e)| {
                format!(
                    "  v{}[{}] -- \"{}\" --> v{}[{}]\n",
                    e.0,
                    self.data_string(e.0),
                    format!("e<sub>{}</sub>: ", i)
                        + match e.2 {
                            "" => "&nbsp;",
                            "#" => "&nbsp;#&nbsp;",
                            _ => e.2,
                        },
                    e.1,
                    self.data_string(e.1)
                )
            }))
            .collect()
    }

    fn data_string(&self, v: usize) -> String {
        let content = self.vertices[v]
            .map(|d| format!("({}, {})", d.0, d.1))
            .unwrap_or("&nbsp;".repeat(8));
        format!("\"v<sub>{}</sub><br>{}\"", v, content)
    }

    pub(super) fn vertex_count(&self) -> usize {
        self.edges.len() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suffix_tree_for_quran() {
        let mut t = SuffixTree::new();
        for (i, (_, _, q)) in crate::quran::iter().enumerate() {
            t.construct(i, q);
        }
        // println!("{}", t.to_mermaid());
        assert_eq!(t.edges.len(), 116_076);
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
