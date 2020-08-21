use crate::list::List;

mod list {
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::sync::{Arc, RwLock, RwLockReadGuard, Weak};

    struct Node<T> {
        data: T,
        next: Option<Arc<RwLock<Node<T>>>>,
        prev: Option<Weak<RwLock<Node<T>>>>,
    }

    impl<T> Node<T> {
        fn new(data: T) -> Self {
            Node {
                data,
                next: None,
                prev: None,
            }
        }

        fn add_after(node: &mut Arc<RwLock<Node<T>>>, data: T) -> Arc<RwLock<Node<T>>> {
            let new_node = Arc::new(RwLock::new(Node::new(data)));

            if let Some(next) = &RwLock::read(&node).unwrap().next {
                RwLock::write(next).unwrap().prev = Some(Arc::downgrade(&new_node));
                RwLock::write(&new_node).unwrap().next = Some(Arc::clone(next));
            }

            RwLock::write(node).unwrap().next = Some(new_node.clone());
            new_node
        }
    }

    #[derive(Default)]
    struct ListInternal<T> {
        first: Option<Arc<RwLock<Node<T>>>>,
        last: Option<Arc<RwLock<Node<T>>>>,
    }

    impl<T> ListInternal<T> {
        fn new() -> Self {
            ListInternal {
                first: None,
                last: None,
            }
        }

        fn append(&mut self, data: T) {
            if self.last.is_some() {
                self.last = Some(Node::add_after(self.last.as_mut().unwrap(), data));
            } else {
                let new_node = Arc::new(RwLock::new(Node::new(data)));
                self.first = Some(new_node.clone());
                self.last = Some(new_node);
            }
        }
    }

    #[derive(Default)]
    pub struct List<T>(RwLock<ListInternal<T>>);

    impl<T> List<T> {
        pub fn new() -> Self {
            List(RwLock::new(ListInternal::new()))
        }

        pub fn append(&self, data: T) {
            let mut list = self.0.write().unwrap();
            list.append(data);
        }
    }

    impl<'a, T> IntoIterator for &'a List<T> {
        type Item = ListItem<'a, T>;
        type IntoIter = ListIter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            ListIter {
                list: self,
                node: self.0.read().unwrap().first.clone(),
            }
        }
    }

    pub struct ListIter<'a, T> {
        list: &'a List<T>,
        node: Option<Arc<RwLock<Node<T>>>>,
    }

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = ListItem<'a, T>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.node.is_some() {
                let list = self.list.0.read().unwrap();
                let data = self.node.as_ref().unwrap().clone();

                self.node = self
                    .node
                    .as_ref()
                    .map(|n| RwLock::read(&n).unwrap().next.clone())
                    .flatten();

                return Some(ListItem { list, data });
            }

            None
        }
    }

    pub struct ListItem<'a, T> {
        list: RwLockReadGuard<'a, ListInternal<T>>,
        data: Arc<RwLock<Node<T>>>,
    }

    impl<'a, T> ListItem<'a, T> {
        pub fn peek(&self, f: impl FnOnce(&T)) {
            f(&RwLock::read(&self.data).unwrap().data);
        }
    }
}

fn main() {
    let list = List::new();

    for i in 1..10 {
        list.append(i);
    }

    crossbeam::scope(|scope| {
        scope.spawn(|_| {
            for i in &list {
                std::thread::sleep(core::time::Duration::from_secs(1));
                i.peek(|n| println!("{}", n));
            }
        });

        scope.spawn(|_| {
            for i in 11..20 {
                list.append(i);
            }
        });
    })
    .unwrap();

    for i in &list {
        i.peek(|n| println!("{}", n));
    }
}
