pub(crate) struct WordSuffixIter<'a> {
    text: Option<&'a str>,
}

impl<'a> Iterator for WordSuffixIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let current_text = self.text;
        self.text = current_text.and_then(|t| t.split_once(' ').map(|(_, r)| r.trim_start()));
        current_text
    }
}

pub(crate) trait WordSuffixIterExt {
    fn word_suffixes(&self) -> WordSuffixIter;
}

impl WordSuffixIterExt for str {
    fn word_suffixes(&self) -> WordSuffixIter {
        WordSuffixIter { text: Some(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_suffix_iter() {
        let mut word_suffix_iter = "ab cde  fg   h".word_suffixes();
        assert_eq!(word_suffix_iter.next(), Some("ab cde  fg   h"));
        assert_eq!(word_suffix_iter.next(), Some("cde  fg   h"));
        assert_eq!(word_suffix_iter.next(), Some("fg   h"));
        assert_eq!(word_suffix_iter.next(), Some("h"));
        assert_eq!(word_suffix_iter.next(), None);
    }
}
