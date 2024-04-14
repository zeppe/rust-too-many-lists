#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None }
    }

    pub fn push(&mut self, v: T) {
        self.head = Option::Some(Box::new(Node{elem: v, next: self.head.take()}));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut x) = link {
            link = x.next.take();
        }
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
        self.0.pop()
    }
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> List<T> {
    pub fn iter(& self) -> Iter< T> {
        Iter(self.head.as_deref())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(x) => {
                self.0 = x.next.as_deref();
                Some(&x.elem)
            },
            _ => None
        }
    }
}

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(x) => {
                self.0 = x.next.as_deref_mut();
                Some(&mut x.elem)
            },
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_and_pop() {
        let mut s = List::new();
        assert_eq!(s.peek(), None);
        s.push(5);
        assert_eq!(s.peek(), Some(&5));
        let top = s.peek_mut();
        assert_eq!(top, Some(&mut 5));
        top.map(|x| *x = 4);
        assert_eq!(s.peek(), Some(&4));
        assert_eq!(s.pop(), Some(4));
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.peek(), Some(&1));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut s = List::new();
        s.push(5);
        s.push(1);
        s.push(2);
        let mut i = s.into_iter();
        assert_eq!(i.next(), Some(2));
        assert_eq!(i.next(), Some(1));
        assert_eq!(i.next(), Some(5));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);
    }

    #[test]
    fn iter() {
        let mut s = List::new();
        s.push(5);
        s.push(1);
        s.push(2);
        let mut i = s.iter();
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&5));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);

        let mut j = s.iter();
        assert_eq!(j.next(), Some(&2));
    }
    
    #[test]
    fn iter_mut() {
        let mut s = List::new();
        s.push(5);
        s.push(1);
        s.push(2);
        let mut i = s.iter_mut();
        assert_eq!(i.next().map(|x| core::mem::replace(x, 3)), Some(2));
        assert_eq!(i.next().map(|x| core::mem::replace(x, 2)), Some(1));
        assert_eq!(i.next().map(|x| core::mem::replace(x, 6)), Some(5));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);

        let mut j = s.iter();
        assert_eq!(j.next(), Some(&3));
        assert_eq!(j.next(), Some(&2));
        assert_eq!(j.next(), Some(&6));
        assert_eq!(j.next(), None);
    }

}