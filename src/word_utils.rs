pub(crate) struct WordSuffixIter<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Iterator for WordSuffixIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remaining_text = self.chars.as_str();
        while !matches!(self.chars.next(), Some(' ') | None) {}
        match remaining_text {
            "" => None,
            _ => Some(remaining_text),
        }
    }
}

pub(crate) trait WordSuffixIterExt {
    fn word_suffixes(&self) -> WordSuffixIter;
}

impl WordSuffixIterExt for str {
    fn word_suffixes(&self) -> WordSuffixIter {
        WordSuffixIter {
            chars: self.chars(),
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
