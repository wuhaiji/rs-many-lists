use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Not;
use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    /// front表示前一个节点
    front: Link<T>,
    /// front表示后一个节点
    back: Link<T>,
    elem: T,
}

pub struct LinkedList<T> {
    /// front表示最前一个节点
    front: Link<T>,
    /// back表示最后一个节点
    back: Link<T>,
    len: usize,
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self.iter() {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

// impl<T: PartialEq> PartialEq for LinkedList<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.len() == other.len() && self.iter().eq(other)
//     }
//
//     fn ne(&self, other: &Self) -> bool {
//         self.len() != other.len() || self.iter().ne(other)
//     }
// }

// impl<T: Eq> Eq for LinkedList<T> { }


impl<T: Display> Display for LinkedList<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let iter = self.iter();
        for curr in iter {
            write!(f, "{}", curr)?;
            if self.len == 0 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Iterator for LinkedList<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _p: PhantomData,
        }
    }
    
    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _p: PhantomData,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn peek_mut(&self) -> Option<&mut T> {
        self.front.map(|n| unsafe {
            &mut (*n.as_ptr()).elem
        })
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.front.map(|n| unsafe {
            &(*n.as_ptr()).elem
        })
    }
    
    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(
                Box::into_raw(
                    Box::new(Node {
                        front: None,
                        back: None,
                        elem,
                    })
                )
            );
            
            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }
    
    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(
                Box::into_raw(
                    Box::new(Node {
                        front: None,
                        back: None,
                        elem,
                    })
                )
            );
            
            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|n| {
                let first_node = Box::from_raw(n.as_ptr());
                let result = first_node.elem;
                self.front = first_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }
                self.len -= 1;
                result
            })
        }
    }
    
    pub(crate) fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|n| {
                let last_node = Box::from_raw(n.as_ptr());
                let elem = last_node.elem;
                self.back = last_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None
                } else {
                    self.front = None;
                }
                self.len -= 1;
                elem
            })
        }
    }
    
    
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.front.map(|node| &(*node.as_ptr()).elem)
        }
    }
    
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.front.map(|node| &mut (*node.as_ptr()).elem)
        }
    }
    
    
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.back.map(|node| &(*node.as_ptr()).elem)
        }
    }
    
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.back.map(|node| &mut (*node.as_ptr()).elem)
        }
    }
    
    pub fn clear(&mut self) {
        // Oh look it's drop again
        while let Some(_) = self.pop_front() { }
    }
}


pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T> PartialEq for Iter<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}


pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _p: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.front.map(|n| unsafe {
                self.len -= 1;
                self.front = (*n.as_ptr()).back;
                &(*n.as_ptr()).elem
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // 不能通过节点(因为是双端迭代)判断，只能通过长度判读是否complete
        if self.len == 0 {
            None
        } else {
            self.front.map(|n| unsafe {
                self.len -= 1;
                self.front = (*n.as_ptr()).back;
                &mut (*n.as_ptr()).elem
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> DoubleEndedIterator for LinkedList<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.pop_back()
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.back.map(|n| unsafe {
                self.len -= 1;
                self.back = (*n.as_ptr()).front;
                &(*n.as_ptr()).elem
            })
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.back.map(|n| unsafe {
                self.len -= 1;
                self.back = (*n.as_ptr()).front;
                &mut (*n.as_ptr()).elem
            })
        }
    }
}

impl<T> ExactSizeIterator for LinkedList<T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }
}


#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::default();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_basic_back() {
        let mut list = LinkedList::default();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_back(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_back(10);
        assert_eq!(list.len(), 1);
        list.push_back(20);
        assert_eq!(list.len(), 2);
        list.push_back(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_back(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }


    #[test]
    fn test_basic_back_and_front() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(2);
        println!("{list}");
        list.push_front(1);
        println!("{list}");
        list.push_front(0);
        println!("{list}");
        list.push_back(10);
        println!("{list}");
        list.push_back(11);
        println!("{list}");
        list.push_back(12);
        println!("{list}");

        assert_eq!(list.len(), 6);
        assert_eq!(list.pop_front(), Some(0));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(12));
        assert_eq!(list.pop_back(), Some(11));

        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(10));

        assert_eq!(list.len(), 0);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);

        assert_eq!(list.len(), 0);

        list.push_front(0);

        assert_eq!(list.len(), 1);

        list.pop_front();
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        list = list.iter_mut().map(|v| (*v) * 10).collect();

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&30));
        assert_eq!(iter.next(), Some(&40));
        assert_eq!(iter.next(), Some(&50));
        assert_eq!(iter.next(), None);
    
    
        list.peek_mut().map(|x| (*x) = (*x) * 10);
        assert_eq!(list.peek(), Some(&100));
        list.push_front(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}



