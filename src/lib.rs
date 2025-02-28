pub struct Queue<T> {
    pub items: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.first().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let queue = Queue::<i8>::new();
        assert!(queue.items.is_empty());
    }

    #[test]
    fn enqueue() {
        let mut queue = Queue::new();
        let item = i8::MAX;
        queue.enqueue(item);
        assert_eq!(queue.items.len(), 1);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(i8::MAX);
        queue.enqueue(i8::MIN);

        let item = queue.dequeue();
        assert_eq!(item, Some(i8::MAX));
    }
}
