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

    pub fn get_or_add(&mut self, content: char) -> &mut Self {
        let pos = self.next_harfs.iter().position(|h| h.content == content);
        match pos {
            Some(index) => self.next_harfs.get_mut(index).unwrap(),
            None => {
                self.next_harfs.push(Harf::new(content));
                self.next_harfs.last_mut().unwrap()
            }
        }
    }
}
