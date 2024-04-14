#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

#[derive(Debug)]
enum Link<T> {
    Nil,
    More(Box<Node<T>>)
}

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: Link::Nil }
    }

    pub fn push(&mut self, v: T) {
        self.head = Link::More(Box::new(Node{elem: v, next: std::mem::replace(&mut self.head, Link::Nil)}));
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, Link::Nil) {
            Link::More(x) => {
                self.head = x.next;
                Some(x.elem)
            },
            _ => None,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = std::mem::replace(&mut self.head, Link::Nil);
        while let Link::More(mut x) = link {
            link = std::mem::replace(&mut x.next, Link::Nil);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn push_and_pop() {
        let mut s = List::new();
        s.push(5);
        assert_eq!(s.pop(), Some(5));
        s.push(1);
        s.push(2);
        assert_eq!(s.pop(), Some(2));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
    }
}