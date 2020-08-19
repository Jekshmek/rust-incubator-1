use core::fmt;

use std::fmt::Debug;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::rc::Rc;

struct Unmovable {
    non_structural_field: String,
    _pin: PhantomPinned,
}

impl Unmovable {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = Unmovable {
            non_structural_field: data,
            _pin: PhantomPinned,
        };

        Box::pin(res)
    }

    fn get_non_pinned_field(self: Pin<&mut Self>) -> &mut String {
        unsafe { &mut self.get_unchecked_mut().non_structural_field }
    }
}

mod specific {
    use super::*;

    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }

    impl MutMeSomehow for Unmovable {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            *self.get_non_pinned_field() = "Mutating".into();
        }
    }

    impl<T> MutMeSomehow for Box<T>
    where
        T: Unpin + Default,
    {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let p = Pin::into_inner(self);
            *p.as_mut() = T::default();
        }
    }

    impl<T: Default> MutMeSomehow for Vec<T> {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let p = unsafe { Pin::get_unchecked_mut(self) };

            if let Some(first_element) = p.first_mut() {
                *first_element = T::default();
            }
        }
    }

    impl MutMeSomehow for String {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let s = Pin::into_inner(self);
            s.clear();
        }
    }

    impl MutMeSomehow for &[u8] {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let data = Pin::into_inner(self);

            *data = [0; 3].as_ref();
        }
    }

    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: Debug> SayHi for Box<T> {}

    impl<T: Debug> SayHi for Rc<T> {}

    impl<T: Debug> SayHi for Vec<T> {}

    impl SayHi for String {}

    impl SayHi for &[u8] {}
}

mod generic {
    use super::*;

    pub trait MutMeSomehow {
        fn mut_me_somehow(self: Pin<&mut Self>);
    }

    impl<T> MutMeSomehow for T
    where
        T: Unpin + Default,
    {
        fn mut_me_somehow(self: Pin<&mut Self>) {
            let p = Pin::into_inner(self);
            *p = T::default();
        }
    }

    pub trait SayHi: fmt::Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    impl<T: Debug> SayHi for T {}
}

fn main() {
    let mut v: Vec<u8> = vec![1, 2, 3];

    let p = Pin::new(&mut v);
    generic::SayHi::say_hi(p.as_ref());
    specific::MutMeSomehow::mut_me_somehow(p);
    println!("{:?}", v);

    let mut s = &v[..];
    let p = Pin::new(&mut s);
    specific::MutMeSomehow::mut_me_somehow(p);
    println!("{:?}", s);
}
