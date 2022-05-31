use std::collections::HashMap;

pub type TransliterationMap = HashMap<char, Vec<&'static str>>;

pub fn build_transliteration_map() -> TransliterationMap {
    let mut map = HashMap::new();
    map.insert(' ', vec![""]);
    map.insert('ء', split("' k a i u"));
    map.insert('آ', split("a aa"));
    map.insert('أ', split("a u ' k"));
    map.insert('ؤ', split("' k u"));
    map.insert('إ', split("i"));
    map.insert('ئ', split("' k i"));
    map.insert('ا', split("a i u"));
    map.insert('ب', split("b ba bi bu"));
    map.insert('ة', split("h ta ti tu t"));
    map.insert('ت', split("t ta ti tu"));
    map.insert('ث', split("ts tsa tsi tsu s sa si su"));
    map.insert('ج', split("j ja ji ju"));
    map.insert('ح', split("h ha hi hu ch cha chi chu kh kha khi khu"));
    map.insert('خ', split("kh kho khi khu h ho hi hu kha ha"));
    map.insert('د', split("d da di du"));
    map.insert('ذ', split("d da di du dh dha dhi dhu dz dza dzi dzu"));
    map.insert('ر', split("r ro ri ru ra"));
    map.insert('ز', split("z za zi zu"));
    map.insert('س', split("s sa si su"));
    map.insert('ش', split("s sa si su sy sya syi syu sh sha shi shu"));
    map.insert('ص', split("s so si su sh sho shi shu sa sha"));
    map.insert('ض', split("d do di du dh dho dhi dhu dz dzo dzi dzu"));
    map.insert('ط', split("t to ti tu th tho thi thu ta tha"));
    map.insert('ظ', split("d do di du dh dho dhi dhu dz dzo dzi dzu"));
    map.insert('ع', split("' 'a 'i 'u k a i u"));
    map.insert('غ', split("g go gi gu gh gho ghi ghu ga gha"));
    map.insert('ف', split("f fa fi fu"));
    map.insert('ق', split("k ko ki ku q qo qi qu qa"));
    map.insert('ك', split("k ka ki ku"));
    map.insert('ل', split("l lla la li lu"));
    map.insert('م', split("m ma mi mu"));
    map.insert('ن', split("n na ni nu"));
    map.insert('ه', split("h ha hi hu"));
    map.insert('و', split("w wa wi wu u"));
    map.insert('ى', split("a"));
    map.insert('ي', split("y ya yi yu i iya iyi iyu"));
    map
}

fn split(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}
