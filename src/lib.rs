use std::rc::{Rc, Weak};
use std::cell::RefCell;


type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

struct Node<T> {
  value: T,
  next: Link<T>,
  prev: WeakLink<T>,
}

impl<T> Node<T> {
    pub fn new(item: T) -> Self {
        Self {
            value: item,
            next: None,
            prev: None
        }
    }
}

pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, size:0,}
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, item: T) {
        let new_node = Rc::new(RefCell::new(Node::new(item)));

        if let Some(current_tail) = self.tail.take() {
            new_node.borrow_mut().prev = Some(Rc::downgrade(&current_tail));
            current_tail.borrow_mut().next = Some(new_node.clone()); 
            self.tail = Some(new_node);
        } else {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {  
        self.tail.take().map(|current_tail| {
            self.size -= 1;
            match current_tail.borrow_mut().prev.take() {
                Some(node) => {
                    let updgraded_node = node.upgrade().unwrap();
                    updgraded_node.borrow_mut().next = None;
                    self.tail = Some(updgraded_node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(current_tail).ok().unwrap().into_inner().value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }
}


#[test]
pub fn queue_len_test(){
    let mut q = DoublyLinkedList::new();
 
    q.push(0);
    q.push(1);
    q.push(2);
    q.push(3);
    q.pop();
    assert_eq!(q.len(), 3);
}


#[test]
pub fn queue_pop_test(){
    let mut q = DoublyLinkedList::new();
 
    q.push(0);
    q.push(1);
    let val = q.pop();
    assert_eq!(val, Some(1));
}

