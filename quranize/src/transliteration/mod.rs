mod chars;

use chars::*;

pub(super) fn map(c: char) -> &'static [&'static str] {
    match c {
        SPACE => &[""],

        LETTER_HAMZA => &["", "'", "k"],
        LETTER_ALEF_WITH_MADDA_ABOVE => &["a", "aa"],
        LETTER_ALEF_WITH_HAMZA_ABOVE => &["", "'", "k"],
        LETTER_WAW_WITH_HAMZA_ABOVE => &["", "'", "k"],
        LETTER_ALEF_WITH_HAMZA_BELOW => &["", "'", "k"],
        LETTER_YEH_WITH_HAMZA_ABOVE => &["", "'", "k"],
        LETTER_ALEF => &["a", "aa", "o", "oo", ""],
        LETTER_BEH => &["b"],
        LETTER_TEH_MARBUTA => &["h", "t"],
        LETTER_TEH => &["t"],
        LETTER_THEH => &["ts", "s"],
        LETTER_JEEM => &["j"],
        LETTER_HAH => &["h", "kh", "ch"],
        LETTER_KHAH => &["kh"],
        LETTER_DAL => &["d"],
        LETTER_THAL => &["d", "dh", "dz"],
        LETTER_REH => &["r"],
        LETTER_ZAIN => &["z"],
        LETTER_SEEN => &["s"],
        LETTER_SHEEN => &["s", "sy", "sh"],
        LETTER_SAD => &["s", "sh"],
        LETTER_DAD => &["d", "dh", "dz"],
        LETTER_TAH => &["t", "th"],
        LETTER_ZAH => &["d", "dh", "dz"],
        LETTER_AIN => &["", "'", "k"],
        LETTER_GHAIN => &["g", "gh"],

        TATWEEL => &[""],

        LETTER_FEH => &["f"],
        LETTER_QAF => &["k", "q"],
        LETTER_KAF => &["k"],
        LETTER_LAM => &["l"],
        LETTER_MEEM => &["m"],
        LETTER_NOON => &["n"],
        LETTER_HEH => &["h"],
        LETTER_WAW => &["w", "u", "uu"],
        LETTER_ALEF_MAKSURA => &["a", "aa", "y", "i", "ii"],
        LETTER_YEH => &["y", "i", "ii"],

        FATHATAN => &["an", "on", ""],
        DAMMATAN => &["un"],
        KASRATAN => &["in"],
        FATHA => &["a", "o"],
        DAMMA => &["u"],
        KASRA => &["i"],
        SHADDA => &[],

        HAMZA_ABOVE => &["'", "a"],
        LETTER_SUPERSCRIPT_ALEF => &["a", "aa", "o", "oo"],

        _ => &[""],
    }
}

pub(super) fn contextual_map(context: Option<char>, c: char) -> &'static [&'static str] {
    match (context, c) {
        (None, LETTER_ALEF) => &["u", "i"],
        (Some(DAMMA), LETTER_WAW) => &[""],
        (Some(EMPTY_CENTRE_LOW_STOP), LETTER_ALEF_MAKSURA) => &[""],
        (Some(FATHATAN), LETTER_ALEF_MAKSURA) => &[""],
        (Some(KASRA), LETTER_LAM) => &[""],
        (Some(LETTER_ALEF_MAKSURA), LETTER_SUPERSCRIPT_ALEF) => &[""],
        (Some(LETTER_ALEF), LETTER_LAM) => &[""],
        (Some(LETTER_REH), EMPTY_CENTRE_LOW_STOP) => &["e", "ee"],

        (Some(c), SHADDA) => map(c),
        _ => &[],
    }
}

pub(super) fn single_harf_map(c: char) -> &'static [&'static str] {
    match c {
        LETTER_ALEF => &["alif"],
        LETTER_BEH => &["ba"],
        LETTER_TEH => &["ta"],
        LETTER_THEH => &["tsa", "sa"],
        LETTER_JEEM => &["jim"],
        LETTER_HAH => &["ha", "cha"],
        LETTER_KHAH => &["kho"],
        LETTER_DAL => &["dal"],
        LETTER_THAL => &["dzal", "dhal"],
        LETTER_REH => &["ro"],
        LETTER_ZAIN => &["za"],
        LETTER_SEEN => &["sin"],
        LETTER_SHEEN => &["syin", "shin"],
        LETTER_SAD => &["shod", "shot", "sod", "sot"],
        LETTER_DAD => &["dhod", "dhot", "dzod", "dzot", "dod", "dot"],
        LETTER_TAH => &["tho", "to"],
        LETTER_ZAH => &["dho", "dzo", "do"],
        LETTER_AIN => &["'ain", "ain"],
        LETTER_GHAIN => &["ghoin", "goin", "ghin", "gin"],
        LETTER_FEH => &["fa"],
        LETTER_QAF => &["qof", "kof"],
        LETTER_KAF => &["kaf"],
        LETTER_LAM => &["lam"],
        LETTER_MEEM => &["mim"],
        LETTER_NOON => &["nun"],
        LETTER_HEH => &["ha"],
        LETTER_WAW => &["wawu", "wau"],
        LETTER_YEH => &["ya"],
        _ => &[],
    }
}
