#[allow(dead_code)]
struct List<T> {
    root: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T: Eq> List<T> {
    fn unshift(&mut self, value: T) {
        self.root = Some(Box::new(Node {
            value: value,
            next: ::std::mem::replace(&mut self.root, None),
        }));
    }

    fn push(&mut self, value: T) {
        fn last_node<T>(node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
            if let Some(ref mut _n) = *node {
                last_node(&mut _n.next)
            } else {
                node
            }
        }
        let _node = last_node(&mut self.root);
        *_node = Some(Box::new(Node {
            value: value,
            next: None,
        }));
    }

    fn search(&self, value: T) -> Result<&Option<Box<Node<T>>>, &'static str> {
        fn search_elem<'a, T: Eq>(
            node: &'a Option<Box<Node<T>>>,
            value: T,
        ) -> Result<&'a Option<Box<Node<T>>>, &'static str> {
            if let Some(ref _n) = *node {
                if _n.value == value {
                    return Ok(node);
                } else {
                    return search_elem(&_n.next, value);
                }
            } else {
                return Err("Not found");
            }
        }

        return search_elem(&self.root, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut node = List { root: None };
        node.push(1);
        node.push(1);
        node.push(2);
        node.push(3);
        node.push(5);

        match node.search(2) {
            Ok(i) => {
                if let Some(ref e) = *i {
                    assert_eq!(e.value, 2);
                } else {
                    assert!(false);
                }
            }
            Err(_) => assert!(false),
        }
    }
}
