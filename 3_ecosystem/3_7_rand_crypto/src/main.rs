use rand::distributions::Alphanumeric;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

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

fn new_access_token() -> String {
    let mut rng = ChaCha8Rng::from_entropy();
    (0..64).map(|_| rng.sample(Alphanumeric)).collect()
}

fn main() {
    println!("Implement me!");
}
