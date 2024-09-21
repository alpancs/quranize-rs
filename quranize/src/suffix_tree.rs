use std::collections::{btree_set::Range, BTreeSet};

mod suffix_iter;

type Vertex = Option<usize>;
type Edge<'a> = (usize, usize, &'a str);

pub(super) struct SuffixTree<'a> {
    vertices: Vec<Vertex>,
    edges: BTreeSet<Edge<'a>>,
}

impl<'a> SuffixTree<'a> {
    const EXPECTED_VERTEX_COUNT: usize = 123_014;

    pub(super) fn new() -> Self {
        let mut vertices = Vec::with_capacity(Self::EXPECTED_VERTEX_COUNT);
        vertices.push(None);
        let edges = Default::default();
        Self { vertices, edges }
    }

    pub(super) fn construct(&mut self, s: &'a str) {
        suffix_iter::suffix_iter(s).for_each(|(i, s)| self.construct_suffix(i, 0, s));
    }

    fn construct_suffix(&mut self, i: usize, v: usize, s: &'a str) {
        match { self.edges_from(v) }.find_map(|&e| Some(e).zip(Self::longest_prefix(s, e.2))) {
            Some((e, p)) if p == e.2 => self.construct_suffix(i, e.1, &s[p.len()..]),
            Some((e, p)) => {
                self.edges.remove(&e);
                let w = self.add_vertex(None);
                self.edges.insert((v, w, p));
                self.edges.insert((w, e.1, &e.2[p.len()..]));
                let x = self.add_vertex(Some(i));
                self.edges.insert((w, x, &s[p.len()..]));
            }
            None => {
                let w = self.add_vertex(Some(i));
                self.edges.insert((v, w, s));
            }
        }
    }

    pub(super) fn edges_from(&self, v: usize) -> Range<Edge<'a>> {
        self.edges.range((v, 0, "")..(v + 1, 0, ""))
    }

    fn longest_prefix(s: &'a str, t: &str) -> Option<&'a str> {
        let pairs = s.char_indices().zip(t.char_indices());
        let ((i, x), _) = pairs.take_while(|((_, x), (_, y))| x == y).last()?;
        Some(&s[..(i + x.len_utf8())])
    }

    fn add_vertex(&mut self, v: Vertex) -> usize {
        self.vertices.push(v);
        self.vertices.len() - 1
    }

    pub(super) fn find(&self, v: usize, s: &str) -> Vec<usize> {
        self.edges_from(v)
            .flat_map(|&(_, w, l)| match (s.strip_prefix(l), l.strip_prefix(s)) {
                (_, Some(_)) if !s.is_empty() => self.collect_data(w),
                (Some(s), _) => self.find(w, s),
                _ => vec![],
            })
            .collect()
    }

    fn collect_data(&self, v: usize) -> Vec<usize> {
        let head = self.vertices[v].into_iter();
        let tail = self
            .edges_from(v)
            .flat_map(|&(_, w, _)| self.collect_data(w));
        head.chain(tail).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QURAN_UTHMANI_MIN;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suffix_tree() {
        let mut t = SuffixTree::new();
        t.construct(&QURAN_UTHMANI_MIN[..(QURAN_UTHMANI_MIN.find("\n\n").unwrap() + 2)]);
        assert_eq!(t.vertices.len(), SuffixTree::EXPECTED_VERTEX_COUNT);
        assert_eq!(t.vertices.len(), t.edges.len() + 1);
        assert_eq!(t.collect_data(0).len(), 77_883);
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
