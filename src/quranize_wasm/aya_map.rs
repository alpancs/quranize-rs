use std::collections::HashMap;

pub type Map = HashMap<(u8, u16), &'static str>;

pub fn build_map() -> Map {
    let mut map = HashMap::new();
    for (s, a, t) in crate::quran::simple_plain_iter() {
        map.insert((s, a), t);
    }
    map
}
