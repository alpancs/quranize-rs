use crate::quran::harf::*;

pub(crate) fn map(c: char) -> &'static [&'static str] {
    match c {
        SPACE => &["", "n"],

        LETTER_HAMZA => &["'", "k", "a", "i", "u"],
        LETTER_ALEF_WITH_MADDA_ABOVE => &["a", "'a", "aa"],
        LETTER_ALEF_WITH_HAMZA_ABOVE => &["'", "k", "a", "u", "al"],
        LETTER_WAW_WITH_HAMZA_ABOVE => &["'", "k", "u"],
        LETTER_ALEF_WITH_HAMZA_BELOW => &["i", "il"],
        LETTER_YEH_WITH_HAMZA_ABOVE => &["'", "k", "a", "i"],
        LETTER_ALEF => &["a", "o"],
        LETTER_BEH => &["b", "ba", "bi", "bu"],
        LETTER_TEH_MARBUTA => &["h", "ta", "ti", "tu", "t"],
        LETTER_TEH => &["t", "ta", "ti", "tu"],
        LETTER_THEH => &["ts", "tsa", "tsi", "tsu", "s", "sa", "si", "su"],
        LETTER_JEEM => &["j", "ja", "ji", "ju"],
        LETTER_HAH => &["h", "ha", "hi", "hu", "ch", "cha", "chi", "chu"],
        LETTER_KHAH => &["kh", "kho", "khi", "khu"],
        LETTER_DAL => &["d", "da", "di", "du"],
        LETTER_THAL => &[
            "d", "da", "di", "du", "dh", "dha", "dhi", "dhu", "dz", "dza", "dzi", "dzu",
        ],
        LETTER_REH => &["r", "ro", "ri", "ru", "ra"],
        LETTER_ZAIN => &["z", "za", "zi", "zu"],
        LETTER_SEEN => &["s", "sa", "si", "su"],
        LETTER_SHEEN => &[
            "s", "sa", "si", "su", "sy", "sya", "syi", "syu", "sh", "sha", "shi", "shu",
        ],
        LETTER_SAD => &[
            "s", "so", "si", "su", "sh", "sho", "shi", "shu", "sa", "sha",
        ],
        LETTER_DAD => &[
            "d", "do", "di", "du", "dh", "dho", "dhi", "dhu", "dz", "dzo", "dzi", "dzu",
        ],
        LETTER_TAH => &[
            "t", "to", "ti", "tu", "th", "tho", "thi", "thu", "ta", "tha",
        ],
        LETTER_ZAH => &[
            "d", "do", "di", "du", "dh", "dho", "dhi", "dhu", "dz", "dzo", "dzi", "dzu",
        ],
        LETTER_AIN => &["'", "'a", "'i", "'u", "k", "a", "i", "u"],
        LETTER_GHAIN => &[
            "g", "go", "gi", "gu", "gh", "gho", "ghi", "ghu", "ga", "gha",
        ],
        LETTER_FEH => &["f", "fa", "fi", "fu"],
        LETTER_QAF => &["k", "ko", "ki", "ku", "q", "qo", "qi", "qu", "qa"],
        LETTER_KAF => &["k", "ka", "ki", "ku"],
        LETTER_LAM => &["l", "la", "li", "lu"],
        LETTER_MEEM => &["m", "ma", "mi", "mu"],
        LETTER_NOON => &["n", "na", "ni", "nu"],
        LETTER_HEH => &["h", "ha", "hi", "hu"],
        LETTER_WAW => &["w", "wa", "wi", "wu", "u"],
        LETTER_ALEF_MAKSURA => &["a", "o"],
        LETTER_YEH => &["y", "ya", "yi", "yu", "i", "ii", "iya", "iyi", "iyu"],

        FATHATAN => &["an", "am"],
        DAMMATAN => &["un", "um"],
        KASRATAN => &["in", "im"],
        FATHA => &["a"],
        DAMMA => &["u"],
        KASRA => &["i"],

        TATWEEL | HAMZA_ABOVE | LETTER_SUPERSCRIPT_ALEF => &[""],

        _ => &[],
    }
}

pub(crate) fn contextual_map(c0: char, c1: char) -> &'static [&'static str] {
    match (c0, c1) {
        (SPACE, LETTER_ALEF)
        | (LETTER_BEH, LETTER_ALEF)
        | (LETTER_ALEF, LETTER_LAM)
        | (LETTER_ALEF_WITH_MADDA_ABOVE, LETTER_LAM)
        | (LETTER_WAW, LETTER_ALEF)
        | (LETTER_ALEF_WITH_HAMZA_ABOVE, LETTER_WAW)
        | (LETTER_AIN, LETTER_ALEF) => &[""],
        ('\0', LETTER_ALEF) => &["i", "u"],
        (LETTER_LAM, LETTER_LAM) => &["i"],
        (_, SHADDA) => map(c0),
        _ => &[],
    }
}

pub(crate) fn single_harf_map(c: char) -> &'static [&'static str] {
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
