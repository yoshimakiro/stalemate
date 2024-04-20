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

fn mine_chunk(data: &[u8], target: &[u8], start_nonce: u64, end_nonce: u64, result: Arc<Mutex<Option<(u64, Vec<u8>)>>>, log_interval: u64) {
    let mut nonce = start_nonce;
    let mut hashes_checked = 0;
    let mut last_log_time = std::time::Instant::now();

    while nonce < end_nonce {
        let hash_result = sha256_hash(data, nonce);
        hashes_checked += 1;

        if hash_result.iter().take(target.len()).all(|&byte| byte == 0) {
            let mut result_lock = result.lock().unwrap();
            *result_lock = Some((nonce, hash_result));
            return;
        }

        if hashes_checked % log_interval == 0 {
            let elapsed = last_log_time.elapsed().as_secs_f64();
            println!("Hashes checked: {:.2e}, Current nonce: {:.2e}, Elapsed time: {:.2} seconds", hashes_checked as f64, nonce as f64, elapsed);
            last_log_time = std::time::Instant::now();
        }

        nonce += 1;
    }
}

fn mine_block(data: &[u8], target: &[u8], num_threads: usize, log_interval: u64) -> Option<(u64, Vec<u8>)> {
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
            mine_chunk(&data_clone, &target_clone, start_nonce, end_nonce, result_clone, log_interval);
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
    let target_zeros = std::env::args().nth(1).unwrap_or_else(|| "2".into()).parse::<usize>().expect("Invalid target zeros");
    let target: Vec<u8> = vec![0u8; target_zeros];
    let num_threads = num_cpus::get(); // Use number of logical CPUs as the default number of threads
    let log_interval = 100_000; // Log progress every 100,000 hashes
    let result = mine_block(data, &target, num_threads, log_interval);
    if let Some((nonce, hash_result)) = result {
        let hex_hash: String = hash_result.iter().map(|byte| format!("{:02x}", byte)).collect();
        println!("Nonce found: {:.2e}", nonce as f64);
        println!("Hash: {}", hex_hash);
    } else {
        println!("No nonce found.");
    }
}
