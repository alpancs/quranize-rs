pub struct Harf {
    pub content: char,
    pub next_harfs: Vec<Harf>,
    pub locations: Vec<(u8, u16, u8)>,
}

impl Harf {
    pub fn new(content: char) -> Self {
        Self {
            content,
            next_harfs: Vec::new(),
            locations: Vec::new(),
        }
    }

    pub fn update_tree(&mut self, sura_number: u8, aya_number: u16, aya_text: &str) {
        let aya_chars: Vec<_> = aya_text.chars().collect();
        let mut word_number = 0;
        for i in 0..aya_chars.len() {
            if i == 0 || aya_chars[i - 1] == ' ' {
                word_number += 1;
                let mut node = &mut *self;
                for j in i..aya_chars.len() {
                    node = node.get_or_add(aya_chars[j]);
                    if j == aya_chars.len() - 1 || aya_chars[j + 1] == ' ' {
                        let location = (sura_number, aya_number, word_number);
                        node.locations.push(location);
                    }
                }
            }
        }
    }

    fn get_or_add(&mut self, content: char) -> &mut Self {
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