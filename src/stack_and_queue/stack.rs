pub struct Stack<T: Copy> {
    pub top: usize,
    pub max: usize,
    state: Vec<Option<T>>,
}

impl<T: Copy> Stack<T> {
    pub fn new(max: usize) -> Self {
        Stack {
            top: 0,
            max,
            state: vec![None; max],
        }
    }

    pub fn push(&mut self, value: T) {
        if self.top == self.max {
            panic!("stack overflow");
        }
        self.state[self.top] = Some(value);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.state[self.top]
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn is_full(&self) -> bool {
        self.top == self.max
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack: Stack<i32> = Stack::new(5);
        assert_eq!(stack.is_empty(), true);
        assert_eq!(stack.is_full(), false);
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.top, 0);

        stack.push(1);
        assert_eq!(stack.top, 1);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), false);
        assert_eq!(stack.pop(), Some(1));

        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);
        assert_eq!(stack.top, 5);
        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.is_full(), true);
        assert_eq!(stack.pop(), Some(6));
        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.top, 0);
    }

    #[test]
    #[should_panic]
    fn test_stack_overflow() {
        let mut stack: Stack<i32> = Stack::new(5);
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.push(5);
        stack.push(6);
    }
}
