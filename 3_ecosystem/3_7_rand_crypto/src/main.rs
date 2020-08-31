use std::path::PathBuf;

use rand::{
    distributions::Alphanumeric,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use rand_chacha::ChaCha8Rng;
use sha3::{Digest, Sha3_256};

fn generate_password(elements: impl Into<&str>, len: usize) -> Option<String> {
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

fn get_file_hash(path: impl Into<PathBuf>) -> Option<Vec<u8>> {
    let str = std::fs::read_to_string(path).ok()?;
    let hash = Sha3_256::digest(str.as_bytes()).to_vec();
    Some(hash)
}

fn hash_password(password: impl Into<&str>) -> String {
    let mut hasher = argonautica::Hasher::default();
    hasher.with_password(password).hash().unwrap()
}

fn main() {
    println!("Implement me!");
}
