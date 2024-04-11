type Location = (u8, u16, usize);
type Edge<'a> = (&'a str, Node<'a>);

#[derive(Default)]
pub(super) struct Node<'a> {
    location: Option<Location>,
    edges: Vec<Edge<'a>>,
}

impl<'a> Node<'a> {
    fn new_with_location(l: Location) -> Self {
        Self {
            location: Some(l),
            ..Default::default()
        }
    }
    fn push(&mut self, edge: Edge<'a>) {
        self.edges.push(edge);
    }
}

#[derive(Default)]
pub(super) struct Tree<'a> {
    root: Node<'a>,
}

impl<'a> Tree<'a> {
    pub(super) fn insert(&mut self, mut s: &'a str, l: Location) {
        let mut node = &mut self.root;
        loop {
            let find_res = { node.edges.iter().enumerate() }
                .find_map(|(i, (label, _))| Some(i).zip(common_prefix(s, label)));
            if let Some((i, p)) = find_res {
                let (label, subnode) = &mut node.edges[i];
                if *label != p {
                    let old_node = Node {
                        location: subnode.location.take(),
                        edges: {
                            let mut edges = Vec::with_capacity(subnode.edges.len());
                            edges.append(&mut subnode.edges);
                            edges
                        },
                    };
                    subnode.push((&label[p.len()..], old_node));
                    subnode.push((&s[p.len()..], Node::new_with_location(l)));
                    *label = p;
                    break;
                }
                node = subnode;
                s = &s[p.len()..];
            } else {
                node.edges.push((s, Node::new_with_location(l)));
                break;
            }
        }
    }

    #[cfg(test)]
    pub(super) fn size(&self) -> usize {
        Self::node_size(&self.root)
    }
    #[cfg(test)]
    fn node_size(node: &Node) -> usize {
        1 + { node.edges.iter().map(|(_, n)| Self::node_size(n)) }.sum::<usize>()
    }
}

fn common_prefix<'a>(s1: &'a str, s2: &str) -> Option<&'a str> {
    let diff = { s1.char_indices().zip(s2.char_indices()) }.find(|((_, c1), (_, c2))| c1 != c2);
    match diff {
        Some(((0, _), _)) => None,
        Some(((i, _), _)) => Some(&s1[..i]),
        None => Some(&s1[..s1.len().min(s2.len())]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fmt;

    impl<'a> fmt::Debug for Tree<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt_node(f, &self.root, 0)
        }
    }
    fn fmt_node(f: &mut fmt::Formatter, node: &Node, indent: u8) -> fmt::Result {
        for _ in 0..indent {
            write!(f, "  ")?;
        }
        writeln!(f, "loc={:?}:", node.location)?;
        for (label, subnode) in &node.edges {
            for _ in 0..indent + 1 {
                write!(f, "  ")?;
            }
            writeln!(f, "label={}", label)?;
            fmt_node(f, subnode, indent + 1)?;
        }
        Ok(())
    }

    #[test]
    fn test_tree_insert() {
        let mut t = Tree::default();
        let s = "aabaa$";
        for i in 0..s.len() {
            t.insert(&s[i..], (0, 0, i));
        }
        assert_eq!(3, t.root.edges.len());
        assert_eq!("a", t.root.edges[0].0);
        assert_eq!(None, t.root.edges[0].1.location);
        assert_eq!("baa$", t.root.edges[1].0);
        assert_eq!(Some((0, 0, 2)), t.root.edges[1].1.location);
        assert_eq!("$", t.root.edges[2].0);
        assert_eq!(Some((0, 0, 5)), t.root.edges[2].1.location);
    }

    #[test]
    fn test_common_prefix() {
        assert_eq!(Some("ab"), common_prefix("abc", "aba"));
        assert_eq!(Some("ab"), common_prefix("ab", "aba"));
        assert_eq!(Some("abc"), common_prefix("abc", "abc"));
        assert_eq!(Some("a"), common_prefix("a", "aa$"));
        assert_eq!(Some("a"), common_prefix("aa$", "a"));
        assert_eq!(None, common_prefix("abc", "cba"));
    }
}
