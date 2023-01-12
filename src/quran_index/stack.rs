// Reference: https://rust-unofficial.github.io/too-many-lists/second-final.html

pub(crate) struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem,
            next: self.head.take(),
        }));
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn iter() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        let mut iter = stack.iter_mut();
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek_mut() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek_mut(), None);
        stack.push(1);
        assert_eq!(stack.peek_mut(), Some(&mut 1));
        if let Some(value) = stack.peek_mut() {
            *value = 2;
        }
        assert_eq!(stack.peek_mut(), Some(&mut 2));
    }
}
