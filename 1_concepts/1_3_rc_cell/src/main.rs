use std::collections::LinkedList;
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct GlobalStack<T>(Arc<Mutex<LinkedList<T>>>);

impl<T> GlobalStack<T> {
    fn push(&self, item: T) {
        let mut inner = self.0.lock().unwrap();
        (*inner).push_back(item);
    }

    fn pop(&self) -> Option<T> {
        let mut inner = self.0.lock().unwrap();
        (*inner).pop_back()
    }
}

fn main() {
    let stack = GlobalStack::default();

    stack.push(0);
    stack.push(1);
    stack.push(2);

    println!("{}", stack.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pop() {
        assert_eq!(GlobalStack::<i32>::default().pop(), None);
    }

    #[test]
    fn pop_element() {
        let stack = GlobalStack::default();

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
    }
}
