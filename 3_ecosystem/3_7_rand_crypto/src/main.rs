use rand::seq::IteratorRandom;

fn generate_password(elements: &str, len: usize) -> Option<String> {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| elements.chars().choose(&mut rng))
        .collect()
}

fn main() {
    println!("Implement me!");
}
