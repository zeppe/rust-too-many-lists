use std::rc::Rc;

#[derive(Debug)]
struct Node <T> {
    elem: T,
    next: Option<Rc<Node<T>>>
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Rc<Node<T>>>
}

impl<T> List<T> {
    pub fn new() -> Self { List { head: None } }
    pub fn prepend(&self, el: T) -> List<T> {
        List { head: Some(Rc::new(Node{ elem: el, next: self.head.clone() }))}
    }
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().map(|x| x.next.clone()).flatten()}
    }
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut n = self.head.take();
        while let Some(x) = n {
            if let Ok(mut y) = Rc::try_unwrap(x) {
                n = y.next.take()
            }
            else {
                n = None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    pub fn prepend_tail() {
        let l = List::new();
        assert_eq!(l.head(), None);
        let l = l.prepend(5).prepend(3).prepend(2);
        assert_eq!(l.head(), Some(&2));
        let l = l.tail();
        assert_eq!(l.head(), Some(&3));
        let l = l.tail().tail();
        assert_eq!(l.head(), None);
        let l = l.tail();
        assert_eq!(l.head(), None);
    }

    #[test]
    fn iter() {
        let s = List::new().prepend(5).prepend(1).prepend(2);
        let mut i = s.iter();
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), Some(&5));
        assert_eq!(i.next(), None);
        assert_eq!(i.next(), None);

        let mut j = s.iter();
        assert_eq!(j.next(), Some(&2));
    }
}