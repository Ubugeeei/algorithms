#![allow(dead_code)]

use std::{cmp::Ordering, mem};

#[derive(Debug)]
enum AVLTree<T> {
  Nil,
  Node(Box<AVLTreeNode<T>>),
}

#[derive(Debug)]
struct AVLTreeNode<T> {
  pub value: T,
  pub left: AVLTree<T>,
  pub right: AVLTree<T>,
  balance_factor: i8,
}

impl<T: Ord> AVLTree<T> {
  /*
   * constructor
   */
  fn new(value: Option<T>) -> Self {
    match value {
      Some(value) => AVLTree::Node(Box::new(AVLTreeNode {
        value,
        left: AVLTree::Nil,
        right: AVLTree::Nil,
        balance_factor: 0,
      })),
      None => AVLTree::Nil,
    }
  }
  fn new_with_values(value: Vec<T>) -> Self {
    let mut tree = AVLTree::new(None);
    for v in value {
      tree.insert(v);
    }
    tree
  }

  /**
   * insert
   */
  pub fn insert(&mut self, value: T) -> bool {
    self._insert(value).0
  }
  /// returns: (inserted, deepened)
  fn _insert(&mut self, value: T) -> (bool, bool) {
    let ret = match *self {
      AVLTree::Nil => {
        let node = AVLTreeNode {
          value,
          left: AVLTree::Nil,
          right: AVLTree::Nil,
          balance_factor: 0,
        };
        *self = AVLTree::Node(Box::new(node));
        (true, true)
      }
      AVLTree::Node(ref mut node) => match node.value.cmp(&value) {
        Ordering::Equal => (false, false),
        Ordering::Less => {
          let (inserted, deepened) = node.right._insert(value);
          if deepened {
            let ret = match node.balance_factor {
              -1 => (inserted, false),
              0 => (inserted, true),
              1 => (inserted, false),
              _ => unreachable!(),
            };
            node.balance_factor += 1;
            ret
          } else {
            (inserted, deepened)
          }
        }
        Ordering::Greater => {
          let (inserted, deepened) = node.left._insert(value);
          if deepened {
            let ret = match node.balance_factor {
              -1 => (inserted, false),
              0 => (inserted, true),
              1 => (inserted, false),
              _ => unreachable!(),
            };
            node.balance_factor -= 1;
            ret
          } else {
            (inserted, deepened)
          }
        }
      },
    };
    self.balance();
    ret
  }

  /*
   * balance
   */
  fn balance(&mut self) {
    match *self {
      AVLTree::Nil => (),
      AVLTree::Node(_) => match self.get_node().balance_factor {
        -2 => {
          let lf = self.get_node().left.get_node().balance_factor;
          match lf {
            -1 => {
              let (is_istd, is_dpnd) = (0, 0);
              self.rotate_right();
              self.get_node().right.get_node().balance_factor = is_istd;
              self.get_node().balance_factor = is_dpnd;
            }
            0 => {
              let (is_istd, is_dpnd) = (-1, 1);
              self.rotate_right();
              self.get_node().right.get_node().balance_factor = is_istd;
              self.get_node().balance_factor = is_dpnd;
            }
            1 => {
              let (is_istd, is_dpnd) = match self
                .get_node()
                .left
                .get_node()
                .right
                .get_node()
                .balance_factor
              {
                -1 => (1, 0),
                0 => (0, 0),
                1 => (0, -1),
                _ => unreachable!(),
              };
              self.get_node().left.rotate_left();
              self.rotate_right();
              self.get_node().right.get_node().balance_factor = is_istd;
              self.get_node().left.get_node().balance_factor = is_dpnd;
              self.get_node().balance_factor = 0;
            }
            _ => unreachable!(),
          }
        }
        2 => {
          let lf = self.get_node().right.get_node().balance_factor;
          match lf {
            -1 => {
              let (is_istd, is_dpnd) = match self
                .get_node()
                .right
                .get_node()
                .left
                .get_node()
                .balance_factor
              {
                1 => (-1, 0),
                0 => (0, 0),
                -1 => (0, 1),
                _ => unreachable!(),
              };
              self.get_node().right.rotate_right();
              self.rotate_left();
              self.get_node().left.get_node().balance_factor = is_istd;
              self.get_node().right.get_node().balance_factor = is_dpnd;
              self.get_node().balance_factor = 0;
            }
            0 => {
              let (is_istd, is_dpnd) = (1, -1);
              self.rotate_left();
              self.get_node().left.get_node().balance_factor = is_istd;
              self.get_node().balance_factor = is_dpnd;
            }
            1 => {
              let (is_istd, is_dpnd) = (0, 0);
              self.rotate_left();
              self.get_node().left.get_node().balance_factor = is_istd;
              self.get_node().balance_factor = is_dpnd;
            }
            _ => unreachable!(),
          }
        }
        _ => (),
      },
    }
  }

  /*
   * get node
   */
  fn get_node(&mut self) -> &mut AVLTreeNode<T> {
    match *self {
      AVLTree::Nil => panic!("call on nil node"),
      AVLTree::Node(ref mut v) => v,
    }
  }
  fn get_right(&mut self) -> &mut Self {
    match *self {
      AVLTree::Nil => panic!("call on nil node"),
      AVLTree::Node(ref mut node) => &mut node.right,
    }
  }
  fn get_left(&mut self) -> &mut Self {
    match *self {
      AVLTree::Nil => panic!("call on Ænil node"),
      AVLTree::Node(ref mut node) => &mut node.left,
    }
  }

  /*
   * rotate
   */
  fn rotate_right(&mut self) {
    let mut v = mem::replace(self, AVLTree::Nil);
    let mut left = mem::replace(v.get_left(), AVLTree::Nil);
    let left_right = mem::replace(left.get_right(), AVLTree::Nil);
    *v.get_left() = left_right;
    *left.get_right() = v;
    *self = left;
  }

  fn rotate_left(&mut self) {
    let mut v = mem::replace(self, AVLTree::Nil);
    let mut right = mem::replace(v.get_right(), AVLTree::Nil);
    let right_left = mem::replace(right.get_left(), AVLTree::Nil);
    *v.get_right() = right_left;
    *right.get_left() = v;
    *self = right;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_alvt() {
    {
      let mut avl = AVLTree::new(Some(1));
      avl.insert(2);
      avl.insert(3);
      avl.insert(4);
      avl.insert(5);
      avl.insert(6);
      avl.insert(7);

      dbg!(&avl);
      assert_eq!(avl.get_node().value, 4);
    }

    {
      let mut avl = AVLTree::new_with_values(vec![5, 7, 3, 1, 2, 4, 6]);
      dbg!(&avl);
      assert_eq!(avl.get_node().value, 3);
    }
  }
}
