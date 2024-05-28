use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
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
    fn extend<I: IntoIterator<Item=T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
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

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other.iter())
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self.iter() {
            item.hash(state);
        }
    }
}


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

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut {
            list: self,
            cur: None,
            index: None,
        }
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
        // Oh, look its drop again
        while let Some(_) = self.pop_front() {}
    }
}


pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _p: PhantomData<&'a T>,
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

unsafe impl<T: Send> Send for LinkedList<T> {}

unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}

unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}

unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}


pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

trait OptionExtension<T> {
    fn if_some<F>(&mut self, f: F) where F: FnOnce(&mut T);
}

impl<T> OptionExtension<T> for Option<T> {
    fn if_some<F>(&mut self, f: F) where F: FnOnce(&mut T) {
        match self {
            None => {}
            Some(t) => {
                f(t)
            }
        }
    }
}

impl<'a, T> CursorMut<'a, T> {
    pub fn move_next(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                self.cur = (*cur.as_ptr()).back;
                if self.cur.is_some() {
                    // 这里
                    self.index.as_mut().if_some(|t| {
                        **t += 1;
                    })
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        }else if !self.list.is_empty() {
            // We're at the ghost, and there is a real front, so move to it!
            self.cur = self.list.front;
            self.index = Some(0)
        } else {
            // We're at the ghost, but that's the only element... do nothing.
        }
    }
    
    pub fn move_prev(&mut self) {
        if let Some(cur) = self.cur {
            unsafe {
                // We're on a real element, go to its previous (front)
                self.cur = (*cur.as_ptr()).front;
                if self.cur.is_some() {
                    *self.index.as_mut().unwrap() -= 1;
                } else {
                    // We just walked to the ghost, no more index
                    self.index = None;
                }
            }
        } else if !self.list.is_empty() {
            // We're at the ghost, and there is a real back, so move to it!
            self.cur = self.list.back;
            self.index = Some(self.list.len - 1)
        } else {
            // We're at the ghost, but that's the only element... do nothing.
        }
    }
    
    pub fn index(&self) -> Option<usize> {
        self.index
    }
    
    pub fn current(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur.map(|node| &mut (*node.as_ptr()).elem)
        }
    }
    
    pub fn peek_next(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur
                .and_then(|node| (*node.as_ptr()).back)
                .map(|node| &mut (*node.as_ptr()).elem)
        }
    }
    
    pub fn peek_prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.cur
                .and_then(|node| (*node.as_ptr()).front)
                .map(|node| &mut (*node.as_ptr()).elem)
        }
    }
    
    pub fn split_before(&mut self) -> LinkedList<T> {
        // We have this:
        //
        //     list.front -> A <-> B <-> C <-> D <- list.back
        //                               ^
        //                              cur
        //
        //
        // And we want to produce this:
        //
        //     list.front -> C <-> D <- list.back
        //                   ^
        //                  cur
        //
        //
        //    return.front -> A <-> B <- return.back
        //
        if let Some(cur) = self.cur {
            // We are pointing at a real element, so the list is non-empty.
            unsafe {
                // Current state
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let prev = (*cur.as_ptr()).front;
                
                // What self will become
                let new_len = old_len - old_idx;
                let new_front = self.cur;
                let new_back = self.list.back;
                let new_idx = Some(0);
                
                // What the output will become
                let output_len = old_len - new_len;
                let output_front = self.list.front;
                let output_back = prev;
                
                // Break the links between cur and prev
                if let Some(prev) = prev {
                    (*cur.as_ptr()).front = None;
                    (*prev.as_ptr()).back = None;
                }
                
                // Produce the result:
                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_idx;
                
                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                }
            }
        } else {
            // We're at the ghost, just replace our list with an empty one.
            // No other state needs to be changed.
            std::mem::replace(self.list, LinkedList::new())
        }
    }
    
    pub fn split_after(&mut self) -> LinkedList<T> {
        // We have this:
        //
        //     list.front -> A <-> B <-> C <-> D <- list.back
        //                         ^
        //                        cur
        //
        //
        // And we want to produce this:
        //
        //     list.front -> A <-> B <- list.back
        //                         ^
        //                        cur
        //
        //
        //    return.front -> C <-> D <- return.back
        //
        if let Some(cur) = self.cur {
            // We are pointing at a real element, so the list is non-empty.
            unsafe {
                // Current state
                let old_len = self.list.len;
                let old_idx = self.index.unwrap();
                let next = (*cur.as_ptr()).back;
                
                // What self will become
                let new_len = old_idx + 1;
                let new_back = self.cur;
                let new_front = self.list.front;
                let new_idx = Some(old_idx);
                
                // What the output will become
                let output_len = old_len - new_len;
                let output_front = next;
                let output_back = self.list.back;
                
                // Break the links between cur and next
                if let Some(next) = next {
                    (*cur.as_ptr()).back = None;
                    (*next.as_ptr()).front = None;
                }
                
                // Produce the result:
                self.list.len = new_len;
                self.list.front = new_front;
                self.list.back = new_back;
                self.index = new_idx;
                
                LinkedList {
                    front: output_front,
                    back: output_back,
                    len: output_len,
                }
            }
        } else {
            // We're at the ghost, just replace our list with an empty one.
            // No other state needs to be changed.
            std::mem::replace(self.list, LinkedList::new())
        }
    }
    
    pub fn splice_before(&mut self, mut input: LinkedList<T>) {
        // We have this:
        //
        // input.front -> 1 <-> 2 <- input.back
        //
        // list.front -> A <-> B <-> C <- list.back
        //                     ^
        //                    cur
        //
        //
        // Becoming this:
        //
        // list.front -> A <-> 1 <-> 2 <-> B <-> C <- list.back
        //                                 ^
        //                                cur
        //
        unsafe {
            // We can either `take` the input's pointers or `mem::forget`
            // it. Using `take` is more responsible in case we ever do custom
            // allocators or something that also needs to be cleaned up!
            if input.is_empty() {
                // Input is empty, do nothing.
            } else if let Some(cur) = self.cur {
                // Both lists are non-empty
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();
                
                if let Some(prev) = (*cur.as_ptr()).front {
                    // General Case, no boundaries, just internal fixups
                    (*prev.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(prev);
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                } else {
                    // No prev, we're appending to the front
                    (*cur.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(cur);
                    self.list.front = Some(in_front);
                }
                // Index moves forward by input length
                *self.index.as_mut().unwrap() += input.len;
            } else if let Some(back) = self.list.back {
                // We're on the ghost but non-empty, append to the back
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();
                
                (*back.as_ptr()).back = Some(in_front);
                (*in_front.as_ptr()).front = Some(back);
                self.list.back = Some(in_back);
            } else {
                // We're empty, become the input, remain on the ghost
                std::mem::swap(self.list, &mut input);
            }
            
            self.list.len += input.len;
            // Not necessary but Polite To Do
            input.len = 0;
            
            // Input dropped here
        }
    }
    
    pub fn splice_after(&mut self, mut input: LinkedList<T>) {
        // We have this:
        //
        // input.front -> 1 <-> 2 <- input.back
        //
        // list.front -> A <-> B <-> C <- list.back
        //                     ^
        //                    cur
        //
        //
        // Becoming this:
        //
        // list.front -> A <-> B <-> 1 <-> 2 <-> C <- list.back
        //                     ^
        //                    cur
        //
        unsafe {
            // We can either `take` the input's pointers or `mem::forget`
            // it. Using `take` is more responsible in case we ever do custom
            // allocators or something that also needs to be cleaned up!
            if input.is_empty() {
                // Input is empty, do nothing.
            } else if let Some(cur) = self.cur {
                // Both lists are non-empty
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();
                
                if let Some(next) = (*cur.as_ptr()).back {
                    // General Case, no boundaries, just internal fixups
                    (*next.as_ptr()).front = Some(in_back);
                    (*in_back.as_ptr()).back = Some(next);
                    (*cur.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(cur);
                } else {
                    // No next, we're appending to the back
                    (*cur.as_ptr()).back = Some(in_front);
                    (*in_front.as_ptr()).front = Some(cur);
                    self.list.back = Some(in_back);
                }
                // Index doesn't change
            } else if let Some(front) = self.list.front {
                // We're on the ghost but non-empty, append to the front
                let in_front = input.front.take().unwrap();
                let in_back = input.back.take().unwrap();
                
                (*front.as_ptr()).front = Some(in_back);
                (*in_back.as_ptr()).back = Some(front);
                self.list.front = Some(in_front);
            } else {
                // We're empty, become the input, remain on the ghost
                std::mem::swap(self.list, &mut input);
            }
            
            self.list.len += input.len;
            // Not necessary but Polite To Do
            input.len = 0;
            
            // Input dropped here
        }
    }
}


#[allow(dead_code)]
fn assert_properties() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<LinkedList<i32>>();
    is_sync::<LinkedList<i32>>();


    is_send::<Iter<i32>>();
    is_sync::<Iter<i32>>();

    is_send::<IterMut<i32>>();
    is_sync::<IterMut<i32>>();


    fn linked_list_covariant<'a, T>(x: LinkedList<&'static T>) -> LinkedList<&'a T> { x }
    fn iter_covariant<'i, 'a, T>(x: Iter<'i, &'static T>) -> Iter<'i, &'a T> { x }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

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
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
            .iter().copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }
    
    #[test]
    fn test_cursor_move_peek() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 1));
        assert_eq!(cursor.peek_next(), Some(&mut 2));
        assert_eq!(cursor.peek_prev(), None);
        assert_eq!(cursor.index(), Some(0));
        cursor.move_prev();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.current(), Some(&mut 2));
        assert_eq!(cursor.peek_next(), Some(&mut 3));
        assert_eq!(cursor.peek_prev(), Some(&mut 1));
        assert_eq!(cursor.index(), Some(1));
        
        let mut cursor = m.cursor_mut();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 6));
        assert_eq!(cursor.peek_next(), None);
        assert_eq!(cursor.peek_prev(), Some(&mut 5));
        assert_eq!(cursor.index(), Some(5));
        cursor.move_next();
        assert_eq!(cursor.current(), None);
        assert_eq!(cursor.peek_next(), Some(&mut 1));
        assert_eq!(cursor.peek_prev(), Some(&mut 6));
        assert_eq!(cursor.index(), None);
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.current(), Some(&mut 5));
        assert_eq!(cursor.peek_next(), Some(&mut 6));
        assert_eq!(cursor.peek_prev(), Some(&mut 4));
        assert_eq!(cursor.index(), Some(4));
    }
    
    #[test]
    fn test_cursor_mut_insert() {
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.splice_before(Some(7).into_iter().collect());
        cursor.splice_after(Some(8).into_iter().collect());
        // check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[7, 1, 8, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        cursor.splice_before(Some(9).into_iter().collect());
        cursor.splice_after(Some(10).into_iter().collect());
        check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]);
        
        /* remove_current not impl'd
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), None);
        cursor.move_next();
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(7));
        cursor.move_prev();
        cursor.move_prev();
        cursor.move_prev();
        assert_eq!(cursor.remove_current(), Some(9));
        cursor.move_next();
        assert_eq!(cursor.remove_current(), Some(10));
        check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[1, 8, 2, 3, 4, 5, 6]);
        */
        
        let mut m: LinkedList<u32> = LinkedList::new();
        m.extend([1, 8, 2, 3, 4, 5, 6]);
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        let mut p: LinkedList<u32> = LinkedList::new();
        p.extend([100, 101, 102, 103]);
        let mut q: LinkedList<u32> = LinkedList::new();
        q.extend([200, 201, 202, 203]);
        cursor.splice_after(p);
        cursor.splice_before(q);
        check_links(&m);
        assert_eq!(
            m.iter().cloned().collect::<Vec<_>>(),
            &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
        );
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_prev();
        let tmp = cursor.split_before();
        assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
        m = tmp;
        let mut cursor = m.cursor_mut();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        cursor.move_next();
        let tmp = cursor.split_after();
        assert_eq!(tmp.into_iter().collect::<Vec<_>>(), &[102, 103, 8, 2, 3, 4, 5, 6]);
        check_links(&m);
        assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[200, 201, 202, 203, 1, 100, 101]);
    }
    
    fn check_links<T>(_list: &LinkedList<T>) {
        // would be good to do this!
    }
    
}

