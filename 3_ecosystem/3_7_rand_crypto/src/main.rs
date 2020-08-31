use rand::seq::{IteratorRandom, SliceRandom};

fn generate_password(elements: &str, len: usize) -> Option<String> {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| elements.chars().choose(&mut rng))
        .collect()
}

fn select_rand_val<T>(slice: &[T]) -> Option<&T> {
    let mut rng = rand::thread_rng();
    slice.choose(&mut rng)
}

fn main() {
    println!("Implement me!");
}
