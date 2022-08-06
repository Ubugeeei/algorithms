#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
enum BinarySearchTree<T: Ord> {
  Node {
    value: T,
    left: Box<BinarySearchTree<T>>,
    right: Box<BinarySearchTree<T>>,
  },
  Nil,
}

impl<T: Ord> BinarySearchTree<T> {
  fn new(value: Option<T>) -> Self {
    match value {
      Some(value) => BinarySearchTree::Node {
        value,
        left: Box::new(BinarySearchTree::Nil),
        right: Box::new(BinarySearchTree::Nil),
      },
      None => BinarySearchTree::Nil,
    }
  }

  fn insert(&mut self, value: T) {
    match self {
      BinarySearchTree::Nil => {
        *self = BinarySearchTree::Node {
          value,
          left: Box::new(BinarySearchTree::Nil),
          right: Box::new(BinarySearchTree::Nil),
        }
      }
      BinarySearchTree::Node {
        value: ref mut v,
        left: ref mut l,
        right: ref mut r,
      } => match value.cmp(v) {
        Ordering::Less => l.insert(value),
        Ordering::Greater => r.insert(value),
        Ordering::Equal => (),
      },
    }
  }

  fn includes(&self, value: T) -> bool {
    match self {
      BinarySearchTree::Node {
        value: v,
        left: l,
        right: r,
      } => match value.cmp(v) {
        Ordering::Less => l.includes(value),
        Ordering::Greater => r.includes(value),
        Ordering::Equal => true,
      },
      BinarySearchTree::Nil => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bst_insert() {
    {
      let mut bst = BinarySearchTree::new(None);
      bst.insert(1);
      bst.insert(2);
      bst.insert(3);
      bst.insert(4);
      bst.insert(5);
      bst.insert(6);

      assert_eq!(
        bst,
        BinarySearchTree::Node {
          value: 1,
          right: Box::new(BinarySearchTree::Node {
            value: 2,
            right: Box::new(BinarySearchTree::Node {
              value: 3,
              right: Box::new(BinarySearchTree::Node {
                value: 4,
                right: Box::new(BinarySearchTree::Node {
                  value: 5,
                  right: Box::new(BinarySearchTree::Node {
                    value: 6,
                    right: Box::new(BinarySearchTree::Nil),
                    left: Box::new(BinarySearchTree::Nil),
                  }),
                  left: Box::new(BinarySearchTree::Nil),
                }),
                left: Box::new(BinarySearchTree::Nil),
              }),
              left: Box::new(BinarySearchTree::Nil),
            }),
            left: Box::new(BinarySearchTree::Nil),
          }),
          left: Box::new(BinarySearchTree::Nil),
        }
      );
    }

    {
      let mut bst = BinarySearchTree::new(None);
      bst.insert(15);
      bst.insert(8);
      bst.insert(2);
      bst.insert(10);
      bst.insert(9);
      bst.insert(25);
      bst.insert(20);
      bst.insert(12);
      bst.insert(11);
      bst.insert(30);

      println!("{:?}", bst);
      assert_eq!(
        bst,
        BinarySearchTree::Node {
          value: 15,
          left: Box::new(BinarySearchTree::Node {
            value: 8,
            left:Box::new( BinarySearchTree::Node {
              value: 2,
              left: Box::new(BinarySearchTree::Nil),
              right: Box::new(BinarySearchTree::Nil)
            }),
            right: Box::new(BinarySearchTree::Node {
              value: 10,
              left: Box::new(BinarySearchTree::Node {
                value: 9,
                left: Box::new(BinarySearchTree::Nil),
                right: Box::new(BinarySearchTree::Nil)
              }),
              right: Box::new( BinarySearchTree::Node {
                value: 12,
                left: Box::new(BinarySearchTree::Node {
                  value: 11,
                  left: Box::new(BinarySearchTree::Nil),
                  right: Box::new(BinarySearchTree::Nil)
                }),
                right: Box::new(BinarySearchTree::Nil)
              })
            })
          }),
          right: Box::new(BinarySearchTree::Node {
            value: 25,
            left: Box::new(BinarySearchTree::Node {
              value: 20,
              left: Box::new(BinarySearchTree::Nil),
              right: Box::new(BinarySearchTree::Nil)
            }),
            right: Box::new(BinarySearchTree::Node {
              value: 30,
              left: Box::new(BinarySearchTree::Nil),
              right: Box::new(BinarySearchTree::Nil)
            })
          })
        }
      );
    }
  }

  #[test]
  fn test_bst_includes() {
    let mut bst = BinarySearchTree::new(None);
    bst.insert(15);
    bst.insert(8);
    bst.insert(2);
    bst.insert(10);
    bst.insert(9);
    bst.insert(25);
    bst.insert(20);
    bst.insert(12);
    bst.insert(11);
    bst.insert(30);

    assert!(bst.includes(15));
    assert!(bst.includes(8));
    assert!(bst.includes(2));
    assert!(bst.includes(10));
    assert!(bst.includes(9));
    assert!(bst.includes(25));
    assert!(bst.includes(20));
    assert!(bst.includes(12));
    assert!(bst.includes(11));
    assert!(bst.includes(30));
    assert!(!bst.includes(1));
    assert!(!bst.includes(7));
    assert!(!bst.includes(13));
    assert!(!bst.includes(21));
    assert!(!bst.includes(26));
  }
}
