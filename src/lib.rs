pub struct Queue<T> {
    pub items: Vec<T>,
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
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
}
