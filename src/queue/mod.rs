#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Queue<T> {
    collection: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            collection: Vec::new(),
        }
    }

    fn enqueue(&mut self, item: T) {
        self.collection.push(item)
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.collection.remove(0))
        }
    }

    fn len(&self) -> usize {
        self.collection.len()
    }

    fn is_empty(&mut self) -> bool {
        self.collection.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.collection.first()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Queue;

    #[test]
    fn create_a_new_queue() {
        let q: Queue<i32> = Queue::new();

        assert_eq!(
            q,
            Queue {
                collection: Vec::new()
            }
        );
    }

    #[test]
    fn enqueue_an_item() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(10);

        assert_eq!(
            q,
            Queue {
                collection: vec![10]
            }
        );
    }

    #[test]
    fn dequeue_an_item_in_bounds() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        assert_eq!(
            q,
            Queue {
                collection: vec![10, 20, 30]
            }
        );

        let dequeued_item: Option<i32> = q.dequeue();

        assert_eq!(dequeued_item, Some(10));

        assert_eq!(
            q,
            Queue {
                collection: vec![20, 30]
            }
        );
    }

    #[test]
    fn dequeue_an_item_out_of_bounds() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        assert_eq!(
            q,
            Queue {
                collection: vec![10, 20, 30]
            }
        );

        q.dequeue();
        q.dequeue();
        q.dequeue();

        // Queue is now empty
        assert!(q.is_empty());

        let dequeued_item: Option<i32> = q.dequeue();

        assert_eq!(dequeued_item, None);

        assert_eq!(
            q,
            Queue {
                collection: Vec::new()
            }
        );
    }

    #[test]
    fn peek_into_the_queue() {
        let mut q: Queue<i32> = Queue::new();
        q.enqueue(10);
        q.enqueue(20);
        q.enqueue(30);

        assert_eq!(
            q,
            Queue {
                collection: vec![10, 20, 30]
            }
        );

        assert_eq!(q.peek(), Some(&10));
        assert_eq!(q.len(), 3);

        q.dequeue();
        assert_eq!(q.peek(), Some(&20));
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn queue_is_empty() {
        let mut q: Queue<String> = Queue::new();
        q.enqueue("".to_owned());

        assert_eq!(q.len(), 1);

        q.dequeue();

        assert!(q.is_empty());
    }
}
