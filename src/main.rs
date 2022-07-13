use std::fmt;
use sp_core::hexdisplay::HexDisplay;
use rand::Rng;
use sha2::{Digest, Sha256};

// number of hashes to generate
const NUMBER_OF_HASHES: i32 = 10000;
// slice of 0, x of the sha256 hash
const HASH_SIZE: usize = 2;
// the hashes which have more occurences than the below number
const LOWER_THRESHOLD_OF_COUNTS: i32 = 3;

// size of the initial strings generated
const SIZE_OF_RANDOM_STRING: usize = 10;

fn get_small_rand_str() -> String {
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..SIZE_OF_RANDOM_STRING {
        s.push(rng.gen::<char>());
    }
    s
}

fn get_colliding_hash(input: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(&input);
    let hash = hasher.result();
    let mut hash_bytes_small = [0u8; HASH_SIZE];
    hash_bytes_small.copy_from_slice(&hash[..HASH_SIZE]);
    hash_bytes_small.to_ascii_lowercase()
}

fn get_seed_data() -> Vec<String> {
    let mut v = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..NUMBER_OF_HASHES {
        let tx = tx.clone();
        let handle = std::thread::spawn(move || {
            let s = get_small_rand_str();
            tx.send(s).expect("Could not send over the channel");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..NUMBER_OF_HASHES {
        v.push(rx.recv().unwrap());
    }
    return v;
}

fn get_colliding_hash_data(input: Vec<String>) -> Vec<Vec<u8>> {
    let mut v = Vec::new();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut handles = Vec::new();
    for element in input {
        let tx = tx.clone();
        let handle = std::thread::spawn(move || {
            let hash = get_colliding_hash(element);
            tx.send(hash).expect("Could not send over the channel");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    for _ in 0..NUMBER_OF_HASHES {
        v.push(rx.recv().unwrap());
    }
    return v;
}

#[derive(Debug, Clone)]
struct Collision {
    hash: Vec<u8>,
    count: i32,
}

impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", HexDisplay::from(&self.hash), self.count)
    }
}

fn main() {
    // generate NUMBER_OF_HASHES small rand str and collect into vec, concurrently
    println!("Generating seed data...");
    let seed = get_seed_data();
    println!("Generating colliding hashes...");

    // get  hash for each element from different threads
    let colliding_hash = get_colliding_hash_data(seed);
    println!("Generated colliding hashes");


    println!("Counting colliding hashes...");
    // get number of hashes which have been repeated
    let mut all: Vec<Collision> = Vec::new();
    // seed into struct

    for hash in colliding_hash {
        all.push(Collision {
            hash,
            count: 1,
        });
    }

    // count number of times each hash  in all is repeated and feed into counts vec
    for i in 0..all.len() {
        for j in i+1..all.len() {
            if all[i].hash == all[j].hash {
                all[i].count += 1;
            }
        }
    }
   
    println!("Counted colliding hashes");
    println!("Colliding hashes:");

    for collision in all {
        if collision.count > LOWER_THRESHOLD_OF_COUNTS {
            println!("{}", collision);
        }
    }
}
