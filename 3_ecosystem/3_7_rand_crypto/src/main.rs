use rand::distributions::Alphanumeric;
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use sha3::{Digest, Sha3_256};
use std::path::PathBuf;

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

fn get_file_hash(path: PathBuf) -> Option<Vec<u8>> {
    let str = std::fs::read_to_string(path).ok()?;
    let hash = Sha3_256::digest(str.as_bytes()).to_vec();
    Some(hash)
}

fn main() {
    println!("Implement me!");
}
