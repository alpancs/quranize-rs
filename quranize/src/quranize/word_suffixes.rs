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
        let mut it = "ab cde  fg   h".word_suffixes();
        assert_eq!(Some((0, "ab cde  fg   h")), it.next());
        assert_eq!(Some((3, "cde  fg   h")), it.next());
        assert_eq!(Some((8, "fg   h")), it.next());
        assert_eq!(Some((13, "h")), it.next());
        assert_eq!(None, it.next());

        let (_, suffixes): (Vec<_>, Vec<_>) =
            "يٰأَيُّهَا الَّذينَ ءامَنُوا اتَّقُوا اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ"
                .word_suffixes()
                .unzip();
        assert_eq!(
            suffixes,
            [
                "يٰأَيُّهَا الَّذينَ ءامَنُوا اتَّقُوا اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "الَّذينَ ءامَنُوا اتَّقُوا اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "ءامَنُوا اتَّقُوا اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "اتَّقُوا اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "اللَّهَ حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "حَقَّ تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "تُقاتِهِ وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "وَلا تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "تَموتُنَّ إِلّا وَأَنتُم مُسلِمونَ",
                "إِلّا وَأَنتُم مُسلِمونَ",
                "وَأَنتُم مُسلِمونَ",
                "مُسلِمونَ",
            ]
        );
        let (_, suffixes): (Vec<_>, Vec<_>) =
            "ذٰلِكَ الكِتٰبُ لا رَيبَ ۛ فيهِ ۛ هُدًى لِلمُتَّقينَ".word_suffixes().unzip();
        assert_eq!(
            suffixes,
            [
                "ذٰلِكَ الكِتٰبُ لا رَيبَ ۛ فيهِ ۛ هُدًى لِلمُتَّقينَ",
                "الكِتٰبُ لا رَيبَ ۛ فيهِ ۛ هُدًى لِلمُتَّقينَ",
                "لا رَيبَ ۛ فيهِ ۛ هُدًى لِلمُتَّقينَ",
                "رَيبَ ۛ فيهِ ۛ هُدًى لِلمُتَّقينَ",
                "فيهِ ۛ هُدًى لِلمُتَّقينَ",
                "هُدًى لِلمُتَّقينَ",
                "لِلمُتَّقينَ",
            ]
        );
        let (_, suffixes): (Vec<_>, Vec<_>) = "رَبِّهِم ۖ وَأُولٰئِكَ".word_suffixes().unzip();
        assert_eq!(suffixes, ["رَبِّهِم ۖ وَأُولٰئِكَ", "وَأُولٰئِكَ",]);
        let (_, suffixes): (Vec<_>, Vec<_>) = "الرَّحيمِ\n".word_suffixes().unzip();
        assert_eq!(suffixes, ["الرَّحيمِ\n"]);
    }
}
