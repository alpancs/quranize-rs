pub(crate) struct WordSuffixIter<'a> {
    splitted_str: Option<(&'a str, &'a str)>,
}

impl<'a> Iterator for WordSuffixIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let right_part = self.splitted_str.map(|(_, r)| r);
        self.splitted_str = right_part.and_then(|r| r.split_once(' '));
        right_part
    }
}

pub(crate) trait WordSuffixIterExt {
    fn word_suffixes(&self) -> WordSuffixIter;
}

impl WordSuffixIterExt for str {
    fn word_suffixes(&self) -> WordSuffixIter {
        WordSuffixIter {
            splitted_str: Some(("", self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_suffix_iter() {
        let mut word_suffix_iter = "ab cde fg h".word_suffixes();
        assert_eq!(word_suffix_iter.next(), Some("ab cde fg h"));
        assert_eq!(word_suffix_iter.next(), Some("cde fg h"));
        assert_eq!(word_suffix_iter.next(), Some("fg h"));
        assert_eq!(word_suffix_iter.next(), Some("h"));
        assert_eq!(word_suffix_iter.next(), None);
    }
}
