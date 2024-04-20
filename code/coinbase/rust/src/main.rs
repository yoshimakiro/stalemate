use std::sync::{Arc, Mutex};
use std::thread;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn sha256_hash(data: &[u8], nonce: u64) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.input(data);
    hasher.input(&nonce.to_le_bytes());
    let mut output = vec![0; hasher.output_bytes()];
    hasher.result(&mut output);
    output
}

fn mine_chunk(data: &[u8], target: &[u8], start_nonce: u64, end_nonce: u64, result: Arc<Mutex<Option<(u64, Vec<u8>)>>>) {
    let mut nonce = start_nonce;
    while nonce < end_nonce {
        let hash_result = sha256_hash(data, nonce);
        if hash_result[..target.len()] == target[..] {
            let mut result_lock = result.lock().unwrap();
            *result_lock = Some((nonce, hash_result));
            return;
        }
        nonce += 1;
    }
}

fn mine_block(data: &[u8], target: &[u8], num_threads: usize) -> Option<(u64, Vec<u8>)> {
    let result = Arc::new(Mutex::new(None));
    let nonce_range = u64::MAX / num_threads as u64;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start_nonce = i as u64 * nonce_range;
        let end_nonce = if i == num_threads - 1 {
            u64::MAX
        } else {
            start_nonce + nonce_range
        };
        let data_clone = data.to_owned();
        let target_clone = target.to_owned();
        let result_clone = result.clone();
        let handle = thread::spawn(move || {
            mine_chunk(&data_clone, &target_clone, start_nonce, end_nonce, result_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result_lock = result.lock().unwrap();
    result_lock.clone()
}

fn main() {
    let data = b"Example block header data";
    let target_zeros = 1; // Adjust this value to change the number of leading zeros required in the hash
    let target: Vec<u8> = vec![0u8; target_zeros / 2].into_iter().chain(vec![15u8; (64 - target_zeros) / 2]).collect();
    let num_threads = num_cpus::get(); // Use number of logical CPUs as the default number of threads
    let result = mine_block(data, &target, num_threads);
    if let Some((nonce, hash_result)) = result {
        println!("Nonce found: {} or {:e}", nonce, nonce as f64);
        println!("Hash: {:x?}", hash_result);
    } else {
        println!("No nonce found.");
    }
}
