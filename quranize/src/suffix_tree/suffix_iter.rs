pub(super) fn suffix_iter(s: &str) -> SuffixIter {
    let it = s.char_indices();
    SuffixIter { s, i: 0 }
}

pub(super) struct SuffixIter<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Iterator for SuffixIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let suffix = &self.s[self.i..];
        match suffix {
            "" => None,
            _ => {
                let item = (self.i, suffix);
                let next = suffix
                    .char_indices()
                    .skip_while(non_separator)
                    .find(non_separator);
                match next {
                    Some((offset, _)) => self.i += offset,
                    None => self.i = self.s.len(),
                }
                Some(item)
            }
        }
    }
}

fn non_separator((_, c): &(usize, char)) -> bool {
    !matches!(c, ' ' | '\u{06D6}'..='\u{06DC}' | '\n')
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_suffix_iter() {
        let mut it = suffix_iter("ab cde  fg \n h\n\n");
        assert_eq!(it.next(), Some((0, "ab cde  fg \n h\n\n")));
        assert_eq!(it.next(), Some((3, "cde  fg \n h\n\n")));
        assert_eq!(it.next(), Some((8, "fg \n h\n\n")));
        assert_eq!(it.next(), Some((13, "h\n\n")));
        assert_eq!(it.next(), None);

        let mut it = suffix_iter("x y");
        assert_eq!(it.next(), Some((0, "x y")));
        assert_eq!(it.next(), Some((2, "y")));
        assert_eq!(it.next(), None);
    }
}
