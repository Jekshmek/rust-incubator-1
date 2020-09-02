use std::sync::mpsc;
use std::thread;

use rand::{thread_rng, Rng};
use rayon::prelude::*;

struct Producer(mpsc::Receiver<[[u8; 64]; 64]>);

impl Producer {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            let mut matrix = [[0_u8; 64]; 64];
            for row in matrix.iter_mut() {
                thread_rng().fill(row);
            }

            tx.send(matrix).unwrap();
        });

        Producer(rx)
    }

    fn recv(&self) -> [[u8; 64]; 64] {
        self.0.recv().unwrap()
    }
}

fn main() {
    let producer = Producer::new();

    loop {
        let matrix = producer.recv();

        crossbeam::scope(|scope| {
            let mut results = [0_u64; 2];

            for res in &mut results {
                *res = scope
                    .spawn(|_| {
                        let sum: u64 = matrix
                            .into_par_iter()
                            .map(|x: &[u8; 64]| x.iter().fold(0_u64, |a, b| a + u64::from(*b)))
                            .sum();
                        sum
                    })
                    .join()
                    .unwrap();
            }

            assert_eq!(results[0], results[1]);
            println!("{}", results[0]);
        })
        .unwrap();
    }
}
