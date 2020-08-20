use once_cell::sync::OnceCell;
use rand::prelude::*;
use std::marker::PhantomData;

struct Fact<T>(PhantomData<T>);

impl<T> Fact<T> {
    fn new() -> Self {
        Fact(PhantomData)
    }
}

impl<T> Fact<Vec<T>> {
    fn fact(&self) -> &'static str {
        static INSTANCE: OnceCell<[&'static str; 3]> = OnceCell::new();
        INSTANCE
            .get_or_init(|| {
                [
                    "Vec is heap-allocated.",
                    "Vec may re-allocate on growing.",
                    "Vec may re-allocate on shrinking.",
                ]
            })
            .choose(&mut thread_rng())
            .unwrap()
    }
}

fn main() {
    let v: Fact<Vec<i32>> = Fact::new();
    println!("{}", v.fact());
}
