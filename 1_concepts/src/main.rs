mod list {
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::sync::{Arc, Weak};

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        next: Option<Arc<RefCell<Node<T>>>>,
        prev: Option<Weak<RefCell<Node<T>>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Self {
            Node {
                data,
                next: None,
                prev: None,
            }
        }

        fn insert_after(node: &mut Arc<RefCell<Node<T>>>, data: T) -> Arc<RefCell<Node<T>>> {
            let new_node = Arc::new(RefCell::new(Node::new(data)));

            if let Some(next) = &mut node.borrow_mut().next {
                next.borrow_mut().prev = Some(Arc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(Arc::clone(next));
            }

            node.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(Arc::downgrade(node));

            new_node
        }

        fn insert_before(node: &mut Arc<RefCell<Node<T>>>, data: T) -> Arc<RefCell<Node<T>>> {
            let new_node = Arc::new(RefCell::new(Node::new(data)));

            if let Some(prev) = &mut node.borrow_mut().prev {
                let prev = prev.upgrade().unwrap();
                prev.borrow_mut().next = Some(Arc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Arc::downgrade(&prev));
            }

            node.borrow_mut().prev = Some(Arc::downgrade(&new_node));
            new_node.borrow_mut().next = Some(Arc::clone(node));

            new_node
        }
    }

    #[derive(Default, Debug)]
    struct ListInternal<T> {
        first: Option<Arc<RefCell<Node<T>>>>,
        last: Option<Arc<RefCell<Node<T>>>>,
    }

    impl<T: Debug> ListInternal<T> {
        fn new() -> Self {
            ListInternal {
                first: None,
                last: None,
            }
        }

        fn init(&mut self, data: T) {
            let new_node = Arc::new(RefCell::new(Node::new(data)));
            self.first = Some(new_node.clone());
            self.last = Some(new_node);
        }

        fn push_back(&mut self, data: T) {
            if let Some(last) = &mut self.last {
                *last = Node::insert_after(last, data);
            } else {
                self.init(data);
            }
        }

        fn push_front(&mut self, data: T) {
            if let Some(first) = &mut self.first {
                *first = Node::insert_before(first, data);
            } else {
                self.init(data);
            }
        }

        fn pop_back(&mut self) -> Option<T> {
            if let Some(last) = &mut self.last {
                let last = if let Some(prev) = &RefCell::borrow(&last.clone()).prev {
                    *last = prev.upgrade().unwrap();
                    Option::take(&mut last.borrow_mut().next).unwrap()
                } else {
                    self.first = None;
                    Option::take(&mut self.last).unwrap()
                };

                return Arc::try_unwrap(last)
                    .ok()
                    .map(|node| node.into_inner().data);
            }

            None
        }

        fn pop_front(&mut self) -> Option<T> {
            if let Some(first) = &mut self.first {
                let res = if let Some(next) = &RefCell::borrow(&first.clone()).next {
                    let res = first.clone();
                    *first = next.clone();
                    res
                } else {
                    self.last = None;
                    Option::take(&mut self.first).unwrap()
                };

                return Arc::try_unwrap(res).ok().map(|node| node.into_inner().data);
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn push_pop_test() {
            let mut list = ListInternal::new();
            list.push_back(2);
            list.push_front(1);
            list.push_back(3);

            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_front(), Some(2));
            assert_eq!(list.pop_front(), Some(3));

            list.push_front(2);
            list.push_back(3);
            list.push_front(1);

            assert_eq!(list.pop_back(), Some(3));
            assert_eq!(list.pop_back(), Some(2));
            assert_eq!(list.pop_back(), Some(1));
        }
    }
}

fn main() {}
