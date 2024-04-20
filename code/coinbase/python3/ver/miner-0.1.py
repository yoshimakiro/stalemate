#!/usr/bin/env python3

import hashlib
import sys
import time
import shutil
import secrets



## essential


def get_terminal_width():
    columns, _ = shutil.get_terminal_size()
    return columns

def clear_print(message):
    terminal_width = get_terminal_width()
    print(f"\r{message}" + ' ' * (terminal_width - len(message)), end='\r')

    
def color_leading_zero(nonce):
    nonce_str = str(nonce)
    leading_zeros = len(nonce_str) - len(nonce_str.lstrip('0'))
    if leading_zeros > 0:
        # Color all leading zeros in orange
        colored_zeros = f"\033[38;5;208m{'0' * leading_zeros}\033[0m"
        return f"{colored_zeros}{nonce_str[leading_zeros:]}"
    else:
        return nonce_str

def color_leading_f(nonce):
    nonce_str = str(nonce).lower()  # Ensure it is lowercase to match 'f'
    leading_fs = len(nonce_str) - len(nonce_str.lstrip('f'))
    if leading_fs > 0:
        # Color all leading 'f's in orange
        colored_fs = f"\033[38;5;208m{'f' * leading_fs}\033[0m"
        return f"{colored_fs}{nonce_str[leading_fs:]}"
    else:
        return nonce_str

## 


def sha256_hash(nonce, data="Example block header data"):
    # Prepare the data with the nonce
    input_data = f"{data}{nonce}".encode()  # Combine and encode to bytes
    # Create a SHA-256 hash of the data
    hash_object = hashlib.sha256()
    hash_object.update(input_data)
    return hash_object.hexdigest()

def sha512_hash(nonce, data="Example block header data"):
    # Prepare the data with the nonce
    input_data = f"{data}{nonce}".encode()  # Combine and encode to bytes
    # Create a SHA-256 hash of the data
    hash_object = hashlib.sha512()
    hash_object.update(input_data)
    return hash_object.hexdigest()



def mine_block(parent=sha512_hash((secrets.randbits(256)))):
    start_time = time.time()  # Start the timer
    last_time = start_time
    lowest = parent
    current = sha512_hash((secrets.randbits(256)))
    total_circulation = 0
    new_coins = 0 
    while True:
        # Update the progress based on the natural logarithmic scale and based on the simple fact that randomness ensures next the lowest number is reached the same amount time was taken to get to the last lowest number.
        if lowest < current:
            lowest = current
            #print(f"{nonce} {time.time() - start_time:.2f}s")
            new_coins = time.time() - last_time
            total_circulation += new_coins 
            print(f"\r{color_leading_f(lowest)} {new_coins:.0f} coins:  total circulation:   {total_circulation:.6}")
        #    if lowest < parent: # Stop mining if a valid nonce is found
        #        break;
        current = sha512_hash(current+parent)

if __name__ == '__main__':
    if len(sys.argv) == 1:
        parent = 'a'
    elif len(sys.argv) == 2:
        parent = sys.argv[1] 
        print(f"parent/target")
        print(f"----")
        print(f"\r{color_leading_f(parent)}")
        print(f"----")
    else:
        sys.exit(1)

    mine_block(parent)

