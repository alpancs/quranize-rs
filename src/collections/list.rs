#[derive(Default)]
pub enum List<T> {
    #[default]
    Empty,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    pub fn push(&mut self, e: T) {
        *self = List::Cons(e, Box::new(std::mem::take(self)));
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { list: self }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

pub struct Iter<'a, T> {
    list: &'a List<T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            List::Cons(head, tail) => {
                self.list = tail;
                Some(head)
            }
            _ => None,
        }
    }
}
