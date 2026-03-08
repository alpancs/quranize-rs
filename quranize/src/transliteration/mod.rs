mod chars;

use chars::*;

pub(super) fn map(c: char) -> &'static [&'static str] {
    match c {
        LINE_FEED => &[],
        SPACE => &[""],

        ARABIC_LETTER_HAMZA => &["", "'", "k"],
        ARABIC_LETTER_ALEF_WITH_MADDA_ABOVE => &["a", "aa", "'a", "'aa"],
        ARABIC_LETTER_ALEF_WITH_HAMZA_ABOVE => &["", "'", "k"],
        ARABIC_LETTER_WAW_WITH_HAMZA_ABOVE => &["", "'", "k"],
        ARABIC_LETTER_ALEF_WITH_HAMZA_BELOW => &["", "'", "k"],
        ARABIC_LETTER_YEH_WITH_HAMZA_ABOVE => &["", "'", "k"],
        ARABIC_LETTER_ALEF => &["a", "aa", "o", "oo", ""],
        ARABIC_LETTER_BEH => &["b"],
        ARABIC_LETTER_TEH_MARBUTA => &["h", "t"],
        ARABIC_LETTER_TEH => &["t"],
        ARABIC_LETTER_THEH => &["ts", "s"],
        ARABIC_LETTER_JEEM => &["j"],
        ARABIC_LETTER_HAH => &["h", "kh", "ch"],
        ARABIC_LETTER_KHAH => &["kh"],
        ARABIC_LETTER_DAL => &["d"],
        ARABIC_LETTER_THAL => &["d", "dh", "dz"],
        ARABIC_LETTER_REH => &["r"],
        ARABIC_LETTER_ZAIN => &["z"],
        ARABIC_LETTER_SEEN => &["s"],
        ARABIC_LETTER_SHEEN => &["s", "sy", "sh"],
        ARABIC_LETTER_SAD => &["s", "sh"],
        ARABIC_LETTER_DAD => &["d", "dh", "dz"],
        ARABIC_LETTER_TAH => &["t", "th"],
        ARABIC_LETTER_ZAH => &["d", "dh", "dz"],
        ARABIC_LETTER_AIN => &["", "'", "k"],
        ARABIC_LETTER_GHAIN => &["g", "gh"],

        ARABIC_LETTER_FEH => &["f"],
        ARABIC_LETTER_QAF => &["k", "q"],
        ARABIC_LETTER_KAF => &["k"],
        ARABIC_LETTER_LAM => &["l"],
        ARABIC_LETTER_MEEM => &["m"],
        ARABIC_LETTER_NOON => &["n"],
        ARABIC_LETTER_HEH => &["h"],
        ARABIC_LETTER_WAW => &["w", "u", "uu"],
        ARABIC_LETTER_ALEF_MAKSURA => &["a", "aa", "y", "i", "ii"],
        ARABIC_LETTER_YEH => &["y", "i", "ii"],

        ARABIC_FATHATAN => &["an", "on", ""],
        ARABIC_DAMMATAN => &["un"],
        ARABIC_KASRATAN => &["in"],
        ARABIC_FATHA => &["a", "o"],
        ARABIC_DAMMA => &["u"],
        ARABIC_KASRA => &["i"],
        ARABIC_SHADDA => &[""],
        ARABIC_SUKUN => &[""],

        ARABIC_LETTER_SUPERSCRIPT_ALEF => &["a", "aa", "o", "oo"],

        ARABIC_SMALL_HIGH_LIGATURE_SAD_WITH_LAM_WITH_ALEF_MAKSURA
        | ARABIC_SMALL_HIGH_LIGATURE_QAF_WITH_LAM_WITH_ALEF_MAKSURA
        | ARABIC_SMALL_HIGH_MEEM_INITIAL_FORM
        | ARABIC_SMALL_HIGH_LAM_ALEF
        | ARABIC_SMALL_HIGH_JEEM
        | ARABIC_SMALL_HIGH_THREE_DOTS
        | ARABIC_SMALL_HIGH_SEEN
        | ARABIC_PLACE_OF_SAJDAH => &[""],

        _ => &[""],
    }
}

pub(super) fn contextual_map(context: Option<char>, c: char) -> &'static [&'static str] {
    match (context, c) {
        (None, ARABIC_LETTER_ALEF) => &["u", "i"],
        (Some(ARABIC_DAMMA), ARABIC_LETTER_WAW) => &[""],
        (Some(ARABIC_FATHATAN), ARABIC_LETTER_ALEF_MAKSURA) => &[""],
        (Some(ARABIC_KASRA), ARABIC_LETTER_LAM) => &[""],
        (Some(ARABIC_LETTER_ALEF_MAKSURA), ARABIC_LETTER_SUPERSCRIPT_ALEF) => &[""],
        (Some(ARABIC_FATHA), ARABIC_LETTER_SUPERSCRIPT_ALEF) => &[""],
        (Some(ARABIC_KASRA), ARABIC_LETTER_YEH) => &[""],
        (Some(ARABIC_LETTER_ALEF), ARABIC_LETTER_LAM) => &[""],
        (Some(ARABIC_LETTER_REH), ARABIC_FATHA) => &["e", "ee"],

        (Some(cc), ARABIC_SHADDA) => map(cc),

        _ => &[],
    }
}

pub(super) fn harf_muqottoah_map(c: char) -> &'static [&'static str] {
    match c {
        ARABIC_LETTER_ALEF => &["alif"],
        ARABIC_LETTER_HAH => &["ha", "cha"],
        ARABIC_LETTER_REH => &["ro"],
        ARABIC_LETTER_SEEN => &["sin"],
        ARABIC_LETTER_SAD => &["shod", "shot", "sod", "sot"],
        ARABIC_LETTER_TAH => &["tho", "to"],
        ARABIC_LETTER_AIN => &["'ain", "ain"],
        ARABIC_LETTER_QAF => &["qof", "kof"],
        ARABIC_LETTER_KAF => &["kaf"],
        ARABIC_LETTER_LAM => &["lam"],
        ARABIC_LETTER_MEEM => &["mim"],
        ARABIC_LETTER_NOON => &["nun"],
        ARABIC_LETTER_HEH => &["ha"],
        ARABIC_LETTER_YEH => &["ya"],

        _ => &[],
    }
}
