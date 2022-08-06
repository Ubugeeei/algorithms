#![allow(dead_code)]

#[derive(Debug)]
struct BinarySearchTree<T: Ord> {
  root: BinarySearchTreeNode<T>,
}

#[derive(Debug, PartialEq)]
enum BinarySearchTreeNode<T: Ord> {
  Nil,
  Node {
    value: T,
    left: Box<Self>,
    right: Box<Self>,
  },
}

impl<T: Ord> BinarySearchTree<T> {
  pub fn new() -> Self {
    Self {
      root: BinarySearchTreeNode::Nil,
    }
  }

  /*
   * insert
   */
  pub fn insert(&mut self, value: T) {
    let nil = Self::search_nil(&mut self.root, &value);

    *nil = BinarySearchTreeNode::Node {
      value,
      left: Box::new(BinarySearchTreeNode::Nil),
      right: Box::new(BinarySearchTreeNode::Nil),
    };
  }

  /**
   * sort
   */
  pub fn get_sorted_vec(&self) -> Vec<&T> {
    let mut vec = Vec::new();
    Self::_get_sorted_vec(&self.root, &mut vec);
    vec
  }
  fn _get_sorted_vec<'a, 'b>(node: &'a BinarySearchTreeNode<T>, vec: &'b mut Vec<&'a T>) {
    match node {
      BinarySearchTreeNode::Nil => (),
      BinarySearchTreeNode::Node { value, left, right } => {
        Self::_get_sorted_vec(left, vec);
        vec.push(value);
        Self::_get_sorted_vec(right, vec);
      }
    }
  }

  /**
   * search
   */
  pub fn includes(&self, value: &T) -> bool {
    Self::_includes(&self.root, value)
  }
  fn _includes(node: &BinarySearchTreeNode<T>, value: &T) -> bool {
    match node {
      BinarySearchTreeNode::Nil => false,
      BinarySearchTreeNode::Node {
        value: v,
        left,
        right,
      } => {
        if v == value {
          true
        } else {
          Self::_includes(left, value) || Self::_includes(right, value)
        }
      }
    }
  }

  pub fn search_by_range(&self, range: &(T, T)) -> Vec<&T> {
    let mut vec = Vec::new();
    Self::_search_by_range(&self.root, &mut vec, range);
    vec
  }
  fn _search_by_range<'a, 'b>(
    node: &'a BinarySearchTreeNode<T>,
    vec: &'b mut Vec<&'a T>,
    range: &(T, T),
  ) {
    match node {
      BinarySearchTreeNode::Nil => (),
      BinarySearchTreeNode::Node { value, left, right } => {
        if value >= &range.0 {
          Self::_search_by_range(left, vec, range);
        }
        if value >= &range.0 && value <= &range.1 {
          vec.push(value);
        }
        if value <= &range.1 {
          Self::_search_by_range(right, vec, range);
        }
      }
    }
  }

  fn search_nil<'a, 'b>(
    node: &'a mut BinarySearchTreeNode<T>,
    value: &'b T,
  ) -> &'a mut BinarySearchTreeNode<T> {
    match node {
      BinarySearchTreeNode::Nil => node,
      BinarySearchTreeNode::Node {
        value: node_v,
        left,
        right,
      } => {
        if value <= node_v {
          Self::search_nil(left, value)
        } else {
          Self::search_nil(right, value)
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bst_new() {
    let bst: BinarySearchTree<i32> = BinarySearchTree::new();
    assert_eq!(bst.root, BinarySearchTreeNode::Nil);
  }

  #[test]
  fn test_bst_insert() {
    {
      let mut bst = BinarySearchTree::new();
      bst.insert(1);
      bst.insert(2);
      bst.insert(3);
      bst.insert(4);

      assert_eq!(
        bst.root,
        BinarySearchTreeNode::Node {
          value: 1,
          left: Box::new(BinarySearchTreeNode::Nil),
          right: Box::new(BinarySearchTreeNode::Node {
            value: 2,
            left: Box::new(BinarySearchTreeNode::Nil),
            right: Box::new(BinarySearchTreeNode::Node {
              value: 3,
              left: Box::new(BinarySearchTreeNode::Nil),
              right: Box::new(BinarySearchTreeNode::Node {
                value: 4,
                left: Box::new(BinarySearchTreeNode::Nil),
                right: Box::new(BinarySearchTreeNode::Nil),
              }),
            }),
          }),
        }
      );
    }
    {
      let mut bst = BinarySearchTree::new();
      bst.insert(1);
      bst.insert(3);
      bst.insert(2);
      bst.insert(4);

      assert_eq!(
        bst.root,
        BinarySearchTreeNode::Node {
          value: 1,
          left: Box::new(BinarySearchTreeNode::Nil),
          right: Box::new(BinarySearchTreeNode::Node {
            value: 3,
            left: Box::new(BinarySearchTreeNode::Node {
              value: 2,
              left: Box::new(BinarySearchTreeNode::Nil),
              right: Box::new(BinarySearchTreeNode::Nil),
            }),
            right: Box::new(BinarySearchTreeNode::Node {
              value: 4,
              left: Box::new(BinarySearchTreeNode::Nil),
              right: Box::new(BinarySearchTreeNode::Nil)
            }),
          }),
        }
      );
    }
  }

  #[test]
  fn test_bst_includes() {
    let mut bst = BinarySearchTree::new();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    bst.insert(4);
    assert!(bst.includes(&1));
    assert!(bst.includes(&5) == false);
  }

  #[test]
  fn test_bst_sort() {
    let mut bst = BinarySearchTree::new();
    bst.insert(3);
    bst.insert(1);
    bst.insert(4);
    bst.insert(2);

    assert_eq!(bst.get_sorted_vec(), vec![&1, &2, &3, &4]);
  }

  #[test]
  fn test_bst_search_by_range() {
    let mut bst = BinarySearchTree::new();
    bst.insert(3);
    bst.insert(1);
    bst.insert(3);
    bst.insert(4);
    bst.insert(2);
    bst.insert(11);
    bst.insert(12);
    bst.insert(13);
    bst.insert(14);

    assert_eq!(bst.search_by_range(&(1, 5)), vec![&1, &2, &3, &3, &4]);
  }
}
