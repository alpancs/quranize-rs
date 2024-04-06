pub(crate) struct WordSuffixIter<'a> {
    offset: Option<usize>,
    str: Option<&'a str>,
}

impl<'a> Iterator for WordSuffixIter<'a> {
    type Item = (usize, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let str = self.str;
        let offset = self.offset;
        let sep = |c| matches!(c, ' ' | '\u{06D6}'..='\u{06DC}');
        self.str = str.and_then(|t| t.split_once(' ').map(|(_, r)| r.trim_start_matches(sep)));
        self.offset = { offset.zip(str.zip(self.str)) }
            .map(|(offset, (before, after))| offset + (before.len() - after.len()));
        offset.zip(str)
    }
}

pub(crate) trait WordSuffixIterExt {
    fn word_suffixes(&self) -> WordSuffixIter;
}

impl WordSuffixIterExt for str {
    fn word_suffixes(&self) -> WordSuffixIter {
        WordSuffixIter {
            offset: Some(0),
            str: Some(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_word_suffix_iter() {
        let mut word_suffix_iter = "ab cde  fg   h".word_suffixes();
        assert_eq!(word_suffix_iter.next(), Some((0, "ab cde  fg   h")));
        assert_eq!(word_suffix_iter.next(), Some((3, "cde  fg   h")));
        assert_eq!(word_suffix_iter.next(), Some((8, "fg   h")));
        assert_eq!(word_suffix_iter.next(), Some((13, "h")));
        assert_eq!(word_suffix_iter.next(), None);
    }
}
