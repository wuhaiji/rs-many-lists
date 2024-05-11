use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }
    
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }
    
    pub fn push(&mut self, elem: T) {
       
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });
        let mut node = *new_tail;
        let raw_tail: *mut Node<T> = &mut node;
        self.tail = raw_tail;
        // .is_null 会检查是否为 null, 在功能上等价于 `None` 的检查
        if !self.tail.is_null() {
            // 如果 old tail 存在，那将其指向新的 tail
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // 否则让 head 指向新的 tail
            self.head = Some(new_tail);
        }
    }
}

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
    }
}
