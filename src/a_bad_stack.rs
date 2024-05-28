use std::mem;

use crate::a_bad_stack::Link::{Empty, More};

#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let link = mem::replace(&mut self.head, Empty);
        let new_node = Node { elem, next: link };
        self.head = More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Empty) {
            Empty => None,
            More(s) => {
                self.head = s.next;
                Some(s.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Empty);
        while let More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::a_bad_stack::List;

    #[test]
    fn test_push_pop() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
