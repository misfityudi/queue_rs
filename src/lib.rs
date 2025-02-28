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

    pub fn enqueue_items(&mut self, items: Vec<T>) {
        self.items.extend(items);
    }

    pub fn dequeue(&mut self) {
        if !self.items.is_empty() {
            self.items.remove(0);
        }
    }

    pub fn front(&self) -> Option<T> {
        self.items.first().cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
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
    fn enqueue_items() {
        let mut queue = Queue::new();
        let items = vec![1, 2, 3];
        queue.enqueue_items(items);
        assert_eq!(queue.items.len(), 3);
    }

    #[test]
    fn dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(i8::MAX);
        queue.enqueue(i8::MIN);

        queue.dequeue();
        assert_eq!(queue.items[0], i8::MIN);
    }

    #[test]
    fn front() {
        let mut queue = Queue::<i8>::new();
        assert_eq!(queue.front(), None);

        queue.enqueue(i8::MAX);
        queue.enqueue(i8::MIN);
        assert_eq!(queue.front(), Some(i8::MAX));
    }

    #[test]
    fn is_empty() {
        let queue = Queue::<i8>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn size() {
        let mut queue = Queue::<i8>::new();

        queue.enqueue(i8::MAX);
        queue.enqueue(i8::MIN);
        assert_eq!(queue.size(), 2);
    }
}
