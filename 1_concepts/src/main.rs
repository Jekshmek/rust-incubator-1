use crate::list::List;

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
                    first.lock().unwrap().prev = None;
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

        pub fn iter(&self) -> ListIter<T> {
            self.into_iter()
        }
    }

    impl<T> IntoIterator for &List<T> {
        type Item = ListItem<T>;
        type IntoIter = ListIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIter(self.first.clone())
        }
    }

    pub struct ListIter<T>(Option<Arc<Mutex<Node<T>>>>);

    impl<T> Iterator for ListIter<T> {
        type Item = ListItem<T>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut next = self
                .0
                .as_ref()
                .and_then(|node| node.lock().unwrap().next.clone());

            std::mem::swap(&mut self.0, &mut next);

            next.map(ListItem)
        }
    }

    pub struct ListItem<T>(Arc<Mutex<Node<T>>>);

    impl<T> ListItem<T> {
        pub fn peek<E, F: FnOnce(&mut T) -> E>(&self, f: F) -> E {
            f(&mut self.0.lock().unwrap().data)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn push_pop_internal() {
            let mut list = List::default();

            list.push_back(2);
            list.push_front(1);
            list.push_back(3);

            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_back(), Some(3));
            assert_eq!(list.pop_front(), Some(2));
            assert_eq!(list.pop_back(), None);

            list.push_front(2);
            list.push_back(3);
            list.push_front(1);

            assert_eq!(list.pop_back(), Some(3));
            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_back(), Some(2));
            assert_eq!(list.pop_front(), None);
        }

        #[test]
        fn thread_safety() {
            let mut list = List::default();

            list.push_back(0);
            list.push_back(0);
            list.push_back(0);

            for (index, item) in list.iter().enumerate() {
                crossbeam::scope(|scope| {
                    scope.spawn(|_| {
                        std::thread::sleep(core::time::Duration::from_millis(10));
                        for item in &list {
                            item.peek(|node| *node += 1);
                        }
                    });
                })
                .unwrap();

                item.peek(|node| {
                    let old = *node;
                    std::thread::sleep(core::time::Duration::from_millis(30));
                    let new = *node;
                    assert_eq!(old, new);
                    assert_eq!(new, index + 1);
                });
            }

            for item in &list {
                assert!(item.peek(|node| *node == 3));
            }
        }
    }
}

fn main() {
    let mut list = List::default();

    list.push_back(1);
    list.push_front(0);
    list.push_back(2);

    for (index, item) in list.iter().enumerate() {
        assert!(item.peek(|val| *val == index));
    }
}
