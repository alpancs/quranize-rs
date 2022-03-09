use std::collections::HashMap;

pub type QuranizeMap = HashMap<String, Vec<String>>;

pub fn build_quranize_map() -> QuranizeMap {
    let transliteration_map_pairs = [
        ("ء", "' k a i u"),
        ("آ", "a aa"),
        ("أ", "a u ' k"),
        ("ؤ", "' k u"),
        ("إ", "i"),
        ("ئ", "' k i"),
        ("ا", "a i u"),
        ("ب", "b ba bi bu"),
        ("ة", "h ta ti tu t"),
        ("ت", "t ta ti tu"),
        ("ث", "ts tsa tsi tsu s sa si su"),
        ("ج", "j ja ji ju"),
        ("ح", "h ha hi hu ch cha chi chu kh kha khi khu"),
        ("خ", "kh kho khi khu h ho hi hu kha ha"),
        ("د", "d da di du"),
        ("ذ", "d da di du dh dha dhi dhu dz dza dzi dzu"),
        ("ر", "r ro ri ru ra"),
        ("ز", "z za zi zu"),
        ("س", "s sa si su"),
        ("ش", "s sa si su sy sya syi syu sh sha shi shu"),
        ("ص", "s so si su sh sho shi shu sa sha"),
        ("ض", "d do di du dh dho dhi dhu dz dzo dzi dzu"),
        ("ط", "t to ti tu th tho thi thu ta tha"),
        ("ظ", "d do di du dh dho dhi dhu dz dzo dzi dzu"),
        ("ع", "' 'a 'i 'u k a i u"),
        ("غ", "g go gi gu gh gho ghi ghu ga gha"),
        ("ف", "f fa fi fu"),
        ("ق", "k ko ki ku q qo qi qu qa"),
        ("ك", "k ka ki ku"),
        ("ل", "l la li lu"),
        ("م", "m ma mi mu"),
        ("ن", "n na ni nu"),
        ("ه", "h ha hi hu"),
        ("و", "w wa wi wu u"),
        ("ى", "a"),
        ("ي", "y ya yi yu i iya iyi iyu"),
        ("ال", "a l la"),
    ];
    let mut quranize_map = QuranizeMap::new();
    for (quran, alphabets) in transliteration_map_pairs {
        for alphabet in alphabets.split_whitespace() {
            let qurans = quranize_map.entry(alphabet.to_string()).or_default();
            qurans.push(quran.to_string());
        }
    }
    quranize_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quranize_map() {
        let quranize_map = build_quranize_map();
        assert_eq!(quranize_map["ba"], vec!["ب"]);
        assert_eq!(
            quranize_map["ku"],
            "ق ك".split_whitespace().collect::<Vec<_>>()
        );
        assert_eq!(
            quranize_map["'"],
            "ء أ ؤ ئ ع".split_whitespace().collect::<Vec<_>>()
        );
    }
}
