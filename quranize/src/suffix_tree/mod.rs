use std::collections::{btree_set::Range, BTreeSet};

mod suffix_iter;

type Vertex = (Option<Index>, usize, bool);
pub(super) type Index = (usize, usize);
pub(super) type Edge<'a> = (usize, usize, &'a str);

pub(super) struct SuffixTree<'a> {
    pub(super) vertices: Vec<Vertex>,
    pub(super) edges: BTreeSet<Edge<'a>>,
}

impl<'a> SuffixTree<'a> {
    pub(super) fn with_capacity(capacity: usize) -> Self {
        let mut vertices = Vec::with_capacity(capacity);
        vertices.push((None, 0, false));
        let edges = Default::default();
        Self { vertices, edges }
    }

    pub(super) fn construct(&mut self, i: usize, s: &'a str) {
        suffix_iter::suffix_iter(s).for_each(|(j, s)| self.construct_suffix((i, j), 0, s));
    }

    fn construct_suffix(&mut self, i: Index, v: usize, s: &'a str) {
        let edge_prefix_pair = self
            .edges_from(v)
            .find_map(|&(v, w, l)| Some((v, w, l)).zip(Self::longest_prefix(s, l)));
        match edge_prefix_pair {
            Some(((_, w, l), p)) if l.len() == p.len() && s.len() > p.len() => {
                self.construct_suffix(i, w, &s[p.len()..]);
                self.vertices[v].2 |= self.vertices[w].2;
            }
            Some(((v, w, l), p)) => {
                self.edges.remove(&(v, w, l));
                let x = self.add_vertex((None, self.vertices[w].1 + 1, false));
                let y = self.add_vertex((Some(i), 1, contains_harf_muqottoah(i)));
                self.edges.insert((v, x, p));
                self.edges.insert((x, w, &l[p.len()..]));
                self.edges.insert((x, y, &s[p.len()..]));
                self.vertices[x].2 = self.vertices[w].2 || self.vertices[y].2;
                self.vertices[v].2 |= self.vertices[x].2;
            }
            None => {
                let w = self.add_vertex((Some(i), 1, contains_harf_muqottoah(i)));
                self.edges.insert((v, w, s));
                self.vertices[v].2 |= self.vertices[w].2;
            }
        }
        self.vertices[v].1 += 1;
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

    pub(super) fn count_data(&self, v: usize) -> usize {
        self.vertices[v].1
    }

    pub(super) fn find(&self, s: &str, v: usize) -> Vec<Index> {
        self.edges_from(v)
            .flat_map(|&(_, w, l)| match (s.strip_prefix(l), l.strip_prefix(s)) {
                (_, Some(_)) if !s.is_empty() => self.collect_data(w),
                (Some(s), _) => self.find(s, w),
                _ => vec![],
            })
            .collect()
    }

    pub(super) fn collect_data(&self, v: usize) -> Vec<Index> {
        let mut buffer = Vec::with_capacity(self.count_data(v));
        self.collect_data_to_buffer(v, &mut buffer);
        buffer
    }

    fn collect_data_to_buffer(&self, v: usize, buffer: &mut Vec<Index>) {
        self.vertices[v].0.into_iter().for_each(|x| buffer.push(x));
        self.edges_from(v)
            .for_each(|&(_, w, _)| self.collect_data_to_buffer(w, buffer));
    }
}

fn contains_harf_muqottoah(i: Index) -> bool {
    matches!(i, (7, 0) | (293, 0) | (954, 0) | (1364, 0) | (1473, 0))
        || matches!(i, (1596, 0) | (1707, 0) | (1750, 0) | (1802, 0) | (2250, 0))
        || matches!(i, (2348, 0) | (2932, 0) | (3159, 0) | (3252, 0) | (3340, 0))
        || matches!(i, (3409, 0) | (3469, 0) | (3503, 0) | (3705, 0) | (3970, 0))
        || matches!(i, (4133, 0) | (4218, 0) | (4272, 0) | (4325, 0) | (4414, 0))
        || matches!(i, (4473, 0) | (4510, 0) | (4630, 0) | (5271, 0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transliteration::harf_muqottoah_map;
    use crate::{Quranize, AYA_COUNT};
    use pretty_assertions::assert_eq;

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

    #[test]
    fn test_contains_harf_muqottoah() {
        let q = Quranize::new();
        (0..AYA_COUNT)
            .filter(|&i| contains_harf_muqottoah((i, 0)))
            .for_each(|i| {
                let (s, a, q) = q.saqs[i];
                let q = q.split_whitespace().next().unwrap();
                let all_chars_is_muq = q.chars().all(|c| !harf_muqottoah_map(c).is_empty());
                assert!(all_chars_is_muq, "i={}, {}:{} => {}", i, s, a, q);
            });
    }
}
