//! Warning: Using raw pointers in Rust will break your program.
//! The test code will not work properly.

#![allow(dead_code)]

use std::fmt::Debug;

#[derive(PartialEq, Eq, Debug, Clone)]
struct LinkedListNode<T: Copy + Debug> {
  value: T,
  next: *const LinkedListNode<T>,
}

impl<T: Copy + Debug> LinkedListNode<T> {
  fn new(value: T) -> Self {
    LinkedListNode {
      value,
      next: std::ptr::null(),
    }
  }

  fn push(&mut self, value: T) {
    let node = LinkedListNode::new(value);
    self.next = &node as *const LinkedListNode<T>;
  }

  fn shift(&mut self) -> Option<T> {
    if self.next.is_null() {
      None
    } else {
      let node = unsafe { &*self.next };
      let v = node.value;
      *self = node.clone();
      Some(v)
    }
  }

  fn unshift(&mut self, value: T) {
    let mut node = LinkedListNode::new(value);
    node.next = &*self as *const LinkedListNode<T>;
    self.next = &node as *const LinkedListNode<T>;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_ll_row_new_push() {
    let mut ll = LinkedListNode::new(1);

    ll.push(2);
    unsafe {
      assert_eq!((*ll.next).value, 2);
    }
  }
}
