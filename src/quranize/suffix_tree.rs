use std::collections::BTreeSet;

type Vertex = Option<u16>;

type Edge<'a> = (usize, usize, &'a str);

struct SuffixTree<'a> {
    vertices: Vec<Vertex>,
    edges: BTreeSet<Edge<'a>>,
}

impl<'a> SuffixTree<'a> {
    fn new() -> Self {
        Self {
            vertices: vec![None],
            edges: Default::default(),
        }
    }

    fn construct(&mut self, s: &'a str) {
        s.char_indices().for_each(|(i, _)| self.insert(i, &s[i..]));
    }

    fn insert(&mut self, i: usize, subs: &'a str) {
        let reusable_edge = { self.edges.range((0, 0, "")..(1, 0, "")) }
            .find_map(|e| Self::longest_prefix(subs, e.2).map(|p| (*e, p)));
        match reusable_edge {
            Some((e, p)) if e.2 != p => {
                let v = self.add_vertex(None);
                let w = self.add_vertex(Some(i as u16));
                self.edges.remove(&e);
                self.edges.insert((e.0, v, p));
                self.edges.insert((v, e.1, e.2.strip_prefix(p).unwrap()));
                self.edges.insert((v, w, subs.strip_prefix(p).unwrap()));
            }
            Some((e, p)) => {
                let v = self.add_vertex(Some(i as u16));
                self.edges.insert((e.1, v, subs.strip_prefix(p).unwrap()));
            }
            None => {
                let v = self.add_vertex(Some(i as u16));
                self.edges.insert((0, v, subs));
            }
        };
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
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    impl SuffixTree<'_> {
        fn to_mermaid(&self) -> String {
            std::iter::once(String::from("graph TB\n"))
                .chain(self.edges.iter().map(|e| {
                    format!(
                        "  v{}(({})) -- \"{}\" --> v{}(({}))\n",
                        e.0,
                        self.data(e.0),
                        e.2,
                        e.1,
                        self.data(e.1)
                    )
                }))
                .collect()
        }

        fn data(&self, v: usize) -> String {
            self.vertices[v]
                .map(|i| i.to_string())
                .unwrap_or("&nbsp;".to_string())
        }
    }

    #[test]
    fn test_suffix_tree() {
        let mut t = SuffixTree::new();
        t.construct("GATAGACA$");
        println!("{}", t.to_mermaid());
        panic!();
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
