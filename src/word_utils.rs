pub struct WordSuffixIter<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> WordSuffixIter<'a> {
    pub fn from(text: &'a str) -> Self {
        Self {
            chars: text.chars(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_suffix_iter() {
        let mut word_suffix_iter = WordSuffixIter::from("ab cde fg h");
        assert_eq!(word_suffix_iter.next(), Some("ab cde fg h"));
        assert_eq!(word_suffix_iter.next(), Some("cde fg h"));
        assert_eq!(word_suffix_iter.next(), Some("fg h"));
        assert_eq!(word_suffix_iter.next(), Some("h"));
        assert_eq!(word_suffix_iter.next(), None);
    }
}
