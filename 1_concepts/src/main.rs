mod list {
    use std::cell::{Ref, RefCell};
    use std::sync::{Arc, Mutex, MutexGuard, Weak};

    pub struct Node<T> {
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

            if let Some(next) = &mut RefCell::borrow_mut(node).next {
                RefCell::borrow_mut(next).prev = Some(Arc::downgrade(&new_node));
                RefCell::borrow_mut(&new_node).next = Some(Arc::clone(next));
            }

            RefCell::borrow_mut(node).next = Some(new_node.clone());
            RefCell::borrow_mut(&new_node).prev = Some(Arc::downgrade(node));

            new_node
        }

        fn insert_before(node: &mut Arc<RefCell<Node<T>>>, data: T) -> Arc<RefCell<Node<T>>> {
            let new_node = Arc::new(RefCell::new(Node::new(data)));

            if let Some(prev) = &mut RefCell::borrow_mut(node).prev {
                let prev = prev.upgrade().unwrap();
                RefCell::borrow_mut(&prev).next = Some(Arc::clone(&new_node));
                RefCell::borrow_mut(&new_node).prev = Some(Arc::downgrade(&prev));
            }

            RefCell::borrow_mut(node).prev = Some(Arc::downgrade(&new_node));
            RefCell::borrow_mut(&new_node).next = Some(Arc::clone(node));

            new_node
        }
    }

    #[derive(Default)]
    struct ListInternal<T> {
        first: Option<Arc<RefCell<Node<T>>>>,
        last: Option<Arc<RefCell<Node<T>>>>,
    }

    impl<T> ListInternal<T> {
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
                let last = if let Some(prev) = &RefCell::borrow_mut(&last.clone()).prev {
                    *last = prev.upgrade().unwrap();
                    Option::take(&mut RefCell::borrow_mut(&last).next).unwrap()
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
                let res = if let Some(next) = &RefCell::borrow_mut(&first.clone()).next {
                    let res = first.clone();
                    *first = next.clone();
                    first.borrow_mut().prev = None;
                    res
                } else {
                    self.last = None;
                    Option::take(&mut self.first).unwrap()
                };

                return Arc::try_unwrap(res).ok().map(|node| node.into_inner().data);
            }

            None
        }

        fn iter(&self) -> ListIter<T> {
            ListIter(self.first.as_ref().map(|node| RefCell::borrow(node)))
        }
    }

    pub struct ListIter<'a, T>(Option<Ref<'a, Node<T>>>);

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = Ref<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            let mut is_there_next = true;

            if let Some(node) = &self.0 {
                let next = unsafe {
                    Ref::map(Ref::clone(node), |node| {
                        if let Some(next) = node.next.as_ref() {
                            RefCell::try_borrow_unguarded(next).unwrap()
                        } else {
                            is_there_next = false;
                            node
                        }
                    })
                };

                let mut next = Some(next);

                std::mem::swap(&mut self.0, &mut next);

                if !is_there_next {
                    self.0 = None;
                }

                return next.map(|node| Ref::map(node, |node| &node.data));
            }

            None
        }
    }

    #[derive(Default)]
    pub struct List<T>(Mutex<ListInternal<T>>);

    impl<T> List<T> {
        pub fn iter(&self) -> ListGuard<T> {
            ListGuard(self.0.lock().unwrap())
        }

        pub fn push_back(&self, data: T) {
            self.0.lock().unwrap().push_back(data);
        }

        pub fn push_front(&self, data: T) {
            self.0.lock().unwrap().push_front(data);
        }

        pub fn pop_back(&self) -> Option<T> {
            self.0.lock().unwrap().pop_back()
        }

        pub fn pop_front(&self) -> Option<T> {
            self.0.lock().unwrap().pop_front()
        }
    }

    pub struct ListGuard<'a, T>(MutexGuard<'a, ListInternal<T>>);

    impl<'a, 'b: 'a, T> IntoIterator for &'b ListGuard<'a, T> {
        type Item = Ref<'a, T>;
        type IntoIter = ListIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.iter()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn push_pop() {
            let mut list = ListInternal::default();

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
        fn iterator() {
            let mut list = List::default();

            list.push_back(0);
            list.push_back(1);
            list.push_back(2);

            for (index, val) in IntoIterator::into_iter(&list.iter()).enumerate() {
                assert_eq!(index, *val);
            }
        }
    }
}

fn main() {}
