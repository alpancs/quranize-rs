use std::collections::BTreeSet;

type Data = (u16, u16);
type Vertex = Option<Data>;
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

    fn construct(&mut self, id: u16, s: &'a str) {
        { s.char_indices() }.for_each(|(i, _)| self.subconst((id, i as u16), 0, &s[i..]));
    }

    fn subconst(&mut self, d: Data, root: usize, subs: &'a str) {
        let mergeable_edge =
            { self.v_edges(root) }.find_map(|e| Self::longest_prefix(subs, e.2).map(|p| (*e, p)));
        match mergeable_edge {
            Some((e, p)) if e.2 == p => self.subconst(d, e.1, &subs[p.len()..]),
            Some((e, p)) => {
                let v = self.add_vertex(None);
                self.edges.remove(&e);
                self.edges.insert((e.0, v, p));
                self.edges.insert((v, e.1, &e.2[p.len()..]));
                self.subconst(d, v, &subs[p.len()..])
            }
            None => {
                let v = self.add_vertex(Some(d));
                self.edges.insert((root, v, subs));
            }
        }
    }

    fn v_edges(&self, v: usize) -> impl Iterator<Item = &Edge<'a>> {
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
                        match e.2 {
                            "" => "&nbsp;",
                            "#" => "&nbsp;#&nbsp;",
                            c => c,
                        },
                        e.1,
                        self.data(e.1)
                    )
                }))
                .collect()
        }

        fn data(&self, v: usize) -> String {
            self.vertices[v]
                .map(|i| format!("{}:{}", i.0, i.1))
                .unwrap_or("&nbsp;".repeat(3))
        }
    }

    #[test]
    fn test_suffix_tree() {
        let mut t = SuffixTree::new();
        t.construct(0, "GATAGACA$");
        t.construct(1, "CATA#");
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
