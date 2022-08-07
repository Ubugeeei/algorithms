#![allow(dead_code)]

// TODO: implement
struct Rawlink<T> {
  p: *mut T,
}
impl<T> Copy for Rawlink<T> {}
impl<T> Clone for Rawlink<T> {
  fn clone(&self) -> Self {
    Rawlink { p: self.p }
  }
}

struct Node<T> {
  next: Option<Box<Node<T>>>,
  prev: Rawlink<Node<T>>,
  value: T,
}

impl<T> Node<T> {
  fn new(v: T) -> Node<T> {
    todo!("implement!");
  }
}
