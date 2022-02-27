pub struct Harf {
    pub content: char,
    next_harfs: Vec<Harf>,
    locations: Vec<u16>,
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
