import hashlib
import sys
import time
import secrets

def sha256_hash(nonce, data="Example block header data"):
    input_data = f"{data}{nonce}".encode()  # Combine and encode to bytes
    hash_object = hashlib.sha256()
    hash_object.update(input_data)
    return hash_object.hexdigest()

def mine_block(target_zeros):
    start_time = time.time()
    nonce = secrets.randbits(256)
    hash_result = sha256_hash(nonce)
    lowest = hash_result
    target = '0' * target_zeros + 'f' * (64 - target_zeros)

    while True:
        if hash_result < lowest:
            lowest = hash_result
            print(f"{hash_result} {time.time() - start_time:.2f}s")  # Output nonce, hash, and time
            if hash_result < target:
                break
        nonce = secrets.randbits(256)
        hash_result = sha256_hash(nonce)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: ./miner.py <number_of_leading_zeros>")
        sys.exit(1)
    
    leading_zeros = int(sys.argv[1])
    mine_block(leading_zeros)
