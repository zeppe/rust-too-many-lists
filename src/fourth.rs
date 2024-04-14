use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node { elem: elem, next: None, prev: None }
    }
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Rc::new(RefCell::new(Node::new(elem)));
        match self.head.take() {
            Some(x) => {
                x.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(x);
            },
            None => {
                self.tail = Some(new_head.clone());
            }
        }
        self.head = Some(new_head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let res = self.head.take().map(|old_head| {
            if let Some(new_head) = &old_head.borrow_mut().next {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head.clone());
            } else {
                self.tail = None;
            }
            // now old_head is the only Rc
            Rc::try_unwrap(old_head).map(|x| x.into_inner().elem).ok()
        });
        return res?;
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|x| Ref::map(x.borrow(), |x| &x.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|x| RefMut::map(x.borrow_mut(), |x| &mut x.elem))
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Rc::new(RefCell::new(Node::new(elem)));
        match self.tail.take() {
            Some(x) => {
                x.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(x);
            },
            None => {
                self.head = Some(new_tail.clone());
            }
        }
        self.tail = Some(new_tail);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let res = self.tail.take().map(|old_tail| {
            if let Some(new_tail) = &old_tail.borrow_mut().prev {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail.clone());
            } else {
                self.head = None;
            }
            // now old_tail is the only Rc
            Rc::try_unwrap(old_tail).map(|x| x.into_inner().elem).ok()
        });
        return res?;
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|x| Ref::map(x.borrow(), |x| &x.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|x| RefMut::map(x.borrow_mut(), |x| &mut x.elem))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

impl<'a, T> List<T> {
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter(self.head.as_ref().map(|x| x.borrow()))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Ref<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        let ret: Option<Self::Item> = self.0.as_ref().map(|&x| Ref::map(x, |n| &n.elem));
        self.0.take().map(|x| {
            self.0 = x.next.as_ref().map(|m| m.borrow());
        });
        ret
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_pop() {
        let mut list = List::new();

        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek_front() {
        let mut list = List::new();

        // Check empty list behaves right
        assert!(list.peek_front().is_none());

        // Populate list
        list.push_front(1);
        assert_eq!(list.peek_front().map(|x| *x), Some(1));

        list.push_front(2);
        assert_eq!(list.peek_front().map(|x| *x), Some(2));

        list.push_front(3);
        assert_eq!(list.peek_front().map(|x| *x), Some(3));

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.peek_front().map(|x| *x), Some(2));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.peek_front().map(|x| *x), Some(1));

        assert_eq!(list.pop_front(), Some(1));
        assert!(list.peek_front().is_none());
    }

    #[test]
    fn basics_back() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn front_back() {
        let mut list = List::new();

        // Check empty list behaves right
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());

        // Populate list
        list.push_front(1);
        assert_eq!(list.peek_front().map(|x| *x), Some(1));
        assert_eq!(list.peek_back().map(|x| *x), Some(1));

        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.peek_front().map(|x| *x), Some(1));
        assert_eq!(list.peek_back().map(|x| *x), Some(3));

        // Check normal removal
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn borrow_mut() {
        let mut list = List::new();

        // Populate list
        list.push_front(1);
        list.push_back(2);
        list.push_back(3);
        
        list.peek_front_mut().map(|mut x| *x = 6);
        list.peek_back_mut().map(|mut x| *x = 7);

        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(6));
    }
}
