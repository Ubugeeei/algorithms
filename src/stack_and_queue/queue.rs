pub struct Queue<T: Copy> {
    pub lead: usize,
    pub rear: usize,
    pub max: usize,
    free: usize,
    state: Vec<Option<T>>,
}

impl<T: Copy> Queue<T> {
    pub fn new(max: usize) -> Self {
        Queue {
            lead: 0,
            rear: 0,
            max,
            free: max,
            state: vec![None; max],
        }
    }

    pub fn enqueue(&mut self, value: T) {
        if self.free == 0 {
            panic!("queue overflow");
        }

        self.state[self.rear] = Some(value);
        self.rear += 1;
        if self.rear == self.max {
            self.rear = 0;
        }
        self.free -= 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.free == self.max {
            return None;
        }
        self.free += 1;
        self.lead += 1;
        self.state[self.lead - 1]
    }

    pub fn is_empty(&self) -> bool {
        self.lead == 0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue: Queue<i32> = Queue::new(5);
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(1);
        assert_eq!(queue.lead, 0);
        assert_eq!(queue.rear, 1);
        dbg!(&queue.state);
        assert_eq!(queue.dequeue(), Some(1));
        dbg!(&queue.state);

        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));

        queue.enqueue(6);
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
    }
}
