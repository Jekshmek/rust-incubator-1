mod list {
    use std::sync::{Arc, Mutex, Weak};

    struct Node<T> {
        data: T,
        next: Option<Arc<Mutex<Node<T>>>>,
        prev: Option<Weak<Mutex<Node<T>>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Self {
            Node {
                data,
                next: None,
                prev: None,
            }
        }

        fn insert_after(node: &mut Arc<Mutex<Node<T>>>, data: T) -> Arc<Mutex<Node<T>>> {
            let new_node = Arc::new(Mutex::new(Node::new(data)));

            if let Some(next) = &mut node.lock().unwrap().next {
                next.lock().unwrap().prev = Some(Arc::downgrade(&new_node));
                new_node.lock().unwrap().next = Some(Arc::clone(next));
            }

            node.lock().unwrap().next = Some(new_node.clone());
            new_node.lock().unwrap().prev = Some(Arc::downgrade(node));

            new_node
        }

        fn insert_before(node: &mut Arc<Mutex<Node<T>>>, data: T) -> Arc<Mutex<Node<T>>> {
            let new_node = Arc::new(Mutex::new(Node::new(data)));

            if let Some(prev) = &mut node.lock().unwrap().prev {
                let prev = prev.upgrade().unwrap();
                prev.lock().unwrap().next = Some(Arc::clone(&new_node));
                new_node.lock().unwrap().prev = Some(Arc::downgrade(&prev));
            }

            node.lock().unwrap().prev = Some(Arc::downgrade(&new_node));
            new_node.lock().unwrap().next = Some(Arc::clone(node));

            new_node
        }
    }

    #[derive(Default)]
    pub struct List<T> {
        first: Option<Arc<Mutex<Node<T>>>>,
        last: Option<Arc<Mutex<Node<T>>>>,
    }

    impl<T> List<T> {
        pub fn init(&mut self, data: T) {
            let new_node = Arc::new(Mutex::new(Node::new(data)));
            self.first = Some(new_node.clone());
            self.last = Some(new_node);
        }

        pub fn push_back(&mut self, data: T) {
            if let Some(last) = &mut self.last {
                *last = Node::insert_after(last, data);
            } else {
                self.init(data);
            }
        }

        pub fn push_front(&mut self, data: T) {
            if let Some(first) = &mut self.first {
                *first = Node::insert_before(first, data);
            } else {
                self.init(data);
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            if let Some(last) = &mut self.last {
                let last = if let Some(prev) = &last.clone().lock().unwrap().prev {
                    *last = prev.upgrade().unwrap();
                    Option::take(&mut last.lock().unwrap().next).unwrap()
                } else {
                    self.first = None;
                    Option::take(&mut self.last).unwrap()
                };

                return Arc::try_unwrap(last)
                    .ok()
                    .map(|node| node.into_inner().unwrap().data);
            }

            None
        }

        pub fn pop_front(&mut self) -> Option<T> {
            if let Some(first) = &mut self.first {
                let res = if let Some(next) = &first.clone().lock().unwrap().next {
                    let res = first.clone();
                    *first = next.clone();
                    res
                } else {
                    self.last = None;
                    Option::take(&mut self.first).unwrap()
                };

                return Arc::try_unwrap(res)
                    .ok()
                    .map(|node| node.into_inner().unwrap().data);
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn list_internal_push_pop_test() {
            let mut list = List::default();
            list.push_back(2);
            list.push_front(1);
            list.push_back(3);

            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_front(), Some(2));
            assert_eq!(list.pop_front(), Some(3));
            assert_eq!(list.pop_front(), None);

            list.push_front(2);
            list.push_back(3);
            list.push_front(1);

            assert_eq!(list.pop_back(), Some(3));
            assert_eq!(list.pop_back(), Some(2));
            assert_eq!(list.pop_back(), Some(1));
            assert_eq!(list.pop_back(), None);
        }
    }
}

fn main() {}
