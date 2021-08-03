use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

struct Node<T> {
  val: T,
  next: Option<NonNull<Node<T>>>,
  prev: Option<NonNull<Node<T>>>
}

impl<T> Node<T> {
  fn new(t: T) -> Node<T> {
    Node {
      val: t,
      prev: None,
      next: None
    }
  }
}

pub struct LinkedList<T> {
  length: usize,
  start: Option<NonNull<Node<T>>>,
  end: Option<NonNull<Node<T>>>
}

impl<T> Default for LinkedList<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> LinkedList<T> {
  pub fn new() -> Self {
    Self {
      length: 0,
      start: None,
      end: None
    }
  }

  pub fn add(&mut self, val: T) {
    let mut node = Box::new(Node::new(val));
    node.next = None;
    node.prev = self.end;
    let node_ptr = Some(unsafe {NonNull::new_unchecked(Box::into_raw(node))} );
    match self.end {
      Some(pt) => unsafe { (*pt.as_ptr()).next = node_ptr},
      None => self.start = node_ptr
    }
    self.end = node_ptr;
  }

  pub fn get(&mut self, index: i32) -> Option<&T> {
    let mut s = self.start;
    while let Some(pt) = s {
      if index == 0 {
        unsafe { &(*pt.as_ptr()).val }
      } else {
        unsafe { s = (*pt.as_ptr()).next;  }
        index -= 1;
      }
    } else {
      None
    }
  }
}