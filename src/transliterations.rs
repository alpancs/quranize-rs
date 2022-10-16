pub fn map(c: char) -> &'static [&'static str] {
    match c {
        'ء' => &["'", "k", "a", "i", "u"],
        'آ' => &["a", "'a", "aa"],
        'أ' => &["'", "k", "a", "u", "al"],
        'ؤ' => &["'", "k", "u"],
        'إ' => &["i", "il"],
        'ئ' => &["'", "k", "a", "i"],
        'ا' => &["a", "o"],
        'ب' => &["b", "ba", "bi", "bu"],
        'ة' => &["h", "ta", "ti", "tu", "t"],
        'ت' => &["t", "ta", "ti", "tu"],
        'ث' => &["ts", "tsa", "tsi", "tsu", "s", "sa", "si", "su"],
        'ج' => &["j", "ja", "ji", "ju"],
        'ح' => &["h", "ha", "hi", "hu", "ch", "cha", "chi", "chu"],
        'خ' => &["kh", "kho", "khi", "khu"],
        'د' => &["d", "da", "di", "du"],
        'ذ' => &[
            "d", "da", "di", "du", "dh", "dha", "dhi", "dhu", "dz", "dza", "dzi", "dzu",
        ],
        'ر' => &["r", "ro", "ri", "ru", "ra"],
        'ز' => &["z", "za", "zi", "zu"],
        'س' => &["s", "sa", "si", "su"],
        'ش' => &[
            "s", "sa", "si", "su", "sy", "sya", "syi", "syu", "sh", "sha", "shi", "shu",
        ],
        'ص' => &[
            "s", "so", "si", "su", "sh", "sho", "shi", "shu", "sa", "sha",
        ],
        'ض' => &[
            "d", "do", "di", "du", "dh", "dho", "dhi", "dhu", "dz", "dzo", "dzi", "dzu",
        ],
        'ط' => &[
            "t", "to", "ti", "tu", "th", "tho", "thi", "thu", "ta", "tha",
        ],
        'ظ' => &[
            "d", "do", "di", "du", "dh", "dho", "dhi", "dhu", "dz", "dzo", "dzi", "dzu",
        ],
        'ع' => &["'", "'a", "'i", "'u", "k", "a", "i", "u"],
        'غ' => &[
            "g", "go", "gi", "gu", "gh", "gho", "ghi", "ghu", "ga", "gha",
        ],
        'ف' => &["f", "fa", "fi", "fu"],
        'ق' => &["k", "ko", "ki", "ku", "q", "qo", "qi", "qu", "qa"],
        'ك' => &["k", "ka", "ki", "ku"],
        'ل' => &["l", "ll", "lla", "la", "li", "lu"],
        'م' => &["m", "ma", "mi", "mu"],
        'ن' => &["n", "na", "ni", "nu"],
        'ه' => &["h", "ha", "hi", "hu"],
        'و' => &["w", "wa", "wi", "wu", "u"],
        'ى' => &["a", "o"],
        'ي' => &["y", "ya", "yi", "yu", "i", "ii", "iya", "iyi", "iyu"],
        ' ' => &["", "n"],
        _ => &[],
    }
}

pub fn contextual_map(prev_c: char, c: char) -> &'static [&'static str] {
    match (prev_c, c) {
        (' ', 'ا') | ('ا', 'ل') | ('و', 'ا') | ('أ', 'و') => &[""],
        ('\0', 'ا') => &["i", "u"],
        _ => &[],
    }
}
