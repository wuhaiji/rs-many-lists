use std::ops::{Deref, DerefMut};
use std::ptr;

type NodeRef<T> = *mut Node<T>;

pub struct List<T> {
    head: NodeRef<T>,
    tail: NodeRef<T>,
}

struct Node<T> {
    elem: T,
    next: NodeRef<T>,
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| unsafe {
            self.next = node.next.as_ref();
            &node.elem
        })
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {
            IterMut {
                next: self.head.as_mut(),
            }
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            let raw = Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }));
            if self.tail.is_null() {
                // .is_null 会检查是否为 null, 在功能上等价于 `None` 的检查
                // 否则让 head 指向新的 tail
                self.head = raw;
            } else {
                // 如果 old tail 存在，那将其指向新的 tail
                (*self.tail).next = raw;
            }
            self.tail = raw;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.elem) }
    }

    pub fn peek_mut(&self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.elem) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        list.push(4);
        assert_eq!(list.pop(), Some(2));
        list.push(5);

        assert_eq!(list.peek(), Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert_eq!(list.peek(), Some(&30));
        assert_eq!(list.pop(), Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        let option = iter.next();
        assert_eq!(option, Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert_eq!(list.pop(), Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert_eq!(list.peek(), Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}
