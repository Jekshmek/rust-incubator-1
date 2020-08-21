mod list {
    use std::cell::RefCell;
    use std::sync::{Arc, Weak};

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

        fn add_after(node: &mut Arc<RefCell<Node<T>>>, data: T) -> Arc<RefCell<Node<T>>> {
            let new_node = Arc::new(RefCell::new(Node::new(data)));

            if let Some(next) = &mut RefCell::borrow_mut(node).next {
                RefCell::borrow_mut(next).prev = Some(Arc::downgrade(&new_node));
                RefCell::borrow_mut(&new_node).next = Some(Arc::clone(next));
            }

            RefCell::borrow_mut(node).next = Some(new_node.clone());
            new_node
        }
    }

    #[derive(Default)]
    struct ListInternal<T> {
        first: Option<Arc<RefCell<Node<T>>>>,
        last: Option<Arc<RefCell<Node<T>>>>,
    }

    impl<T> ListInternal<T> {
        fn new() -> Self {
            ListInternal {
                first: None,
                last: None,
            }
        }

        fn push_back(&mut self, data: T) {
            if self.last.is_some() {
                self.last = Some(Node::add_after(self.last.as_mut().unwrap(), data));
            } else {
                let new_node = Arc::new(RefCell::new(Node::new(data)));
                self.first = Some(new_node.clone());
                self.last = Some(new_node);
            }
        }

        fn push_front(&mut self, data: T) {
            let mut new_node = Node::new(data);

            new_node.next = self.first.clone();
            self.first = Some(Arc::new(RefCell::new(new_node)));

            if self.last.is_none() {
                self.last = self.first.clone();
            }
        }

        fn pop_back(&mut self) -> Option<T> {
            let mut prev = self
                .last
                .as_ref()
                .and_then(|last| RefCell::borrow(last).prev.clone())
                .map(|prev| {
                    let prev = prev.upgrade().unwrap();
                    RefCell::borrow_mut(&prev).next = None;
                    prev
                });

            std::mem::swap(&mut self.last, &mut prev);

            prev.and_then(|node| Arc::try_unwrap(node).ok())
                .map(|ref_cell| ref_cell.into_inner().data)
        }

        fn pop_front(&mut self) -> Option<T> {
            let mut next = self
                .first
                .as_ref()
                .and_then(|first| RefCell::borrow(first).next.clone())
                .map(|next| {
                    RefCell::borrow_mut(&next).prev = None;
                    next
                });

            std::mem::swap(&mut self.first, &mut next);

            next.and_then(|node| Arc::try_unwrap(node).ok())
                .map(|ref_cell| ref_cell.into_inner().data)
        }
    }
}

fn main() {}
