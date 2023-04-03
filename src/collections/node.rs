#[derive(Default)]
pub(crate) struct Node<T> {
    pub(crate) element: T,
    next: Option<Box<List<Self>>>,
}

struct List<T> {
    head: T,
    tail: Option<Box<Self>>,
}

impl<T: PartialEq> Node<T> {
    pub(crate) fn get_mut_or_add(&mut self, element: T) -> &mut Self {
        let pos = self.iter().position(|n| n.element == element);
        match pos {
            Some(n) => self.iter_mut().nth(n).unwrap(),
            None => {
                self.next = Some(Box::new(List {
                    head: Node {
                        element,
                        next: None,
                    },
                    tail: self.next.take(),
                }));
                self.iter_mut().next().unwrap()
            }
        }
    }

    pub(crate) fn iter(&self) -> Iter<Self> {
        Iter {
            next: self.next.as_deref(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<Self> {
        IterMut {
            next: self.next.as_deref_mut(),
        }
    }
}

pub(crate) struct Iter<'a, T> {
    next: Option<&'a List<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|list| {
            self.next = list.tail.as_deref();
            &list.head
        })
    }
}

pub(crate) struct IterMut<'a, T> {
    next: Option<&'a mut List<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|list| {
            self.next = list.tail.as_deref_mut();
            &mut list.head
        })
    }
}
