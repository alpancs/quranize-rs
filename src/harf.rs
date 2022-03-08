pub struct Harf {
    pub content: char,
    pub next_harfs: Vec<Harf>,
    pub locations: Vec<(u8, u16, u8)>,
}

impl Harf {
    pub fn new(content: char) -> Self {
        Self {
            content,
            next_harfs: vec![],
            locations: vec![],
        }
    }
}
