#include <iostream>
#include <sstream>
#include <iomanip>
#include <chrono>
#include <cmath>
#include <openssl/evp.h>
#include <string>

std::string sha256_hash(int nonce, const std::string& data = "Example block header data") {
    std::ostringstream input_stream;
    input_stream << data << nonce;
    unsigned char hash[EVP_MAX_MD_SIZE];
    unsigned int lengthOfHash = 0;

    EVP_MD_CTX* ctx = EVP_MD_CTX_new();
    EVP_DigestInit_ex(ctx, EVP_sha256(), nullptr);
    EVP_DigestUpdate(ctx, input_stream.str().c_str(), input_stream.str().size());
    EVP_DigestFinal_ex(ctx, hash, &lengthOfHash);
    EVP_MD_CTX_free(ctx);

    std::stringstream ss;
    for(unsigned int i = 0; i < lengthOfHash; i++) {
        ss << std::hex << std::setw(2) << std::setfill('0') << (int)hash[i];
    }
    return ss.str();
}

void mine_block(int target_zeros) {
    auto start_time = std::chrono::high_resolution_clock::now();
    int nonce = 0;
    std::string target(target_zeros, '0');
    target += std::string(64 - target_zeros, 'f');

    int nextLogPoint = 1.02;  // Start with 2 and exponentially increase
    while (true) {
        std::string hash_result = sha256_hash(nonce);
        if (hash_result < target) {
            auto end_time = std::chrono::high_resolution_clock::now();
            auto time_taken = std::chrono::duration_cast<std::chrono::seconds>(end_time - start_time).count();
            std::cout << "\rNonce found: " << nonce << std::endl;
            std::cout << "Hash: " << hash_result << std::endl;
            std::cout << "Time taken: " << time_taken << " seconds" << std::endl;
            break;
        }
        nonce++;
        if (nonce > nextLogPoint) {
            std::cout << std::scientific << std::setprecision(2);  // Set output to scientific notation with precision
            std::cout << "\rNonce tested: " << static_cast<double>(nonce) << ", Hash: " << hash_result << std::flush;
            nextLogPoint *= 2;  // Double the threshold for the next update
            std::cout.unsetf(std::ios_base::scientific); // Unset the scientific flag
        }
    }
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <number_of_leading_zeros>" << std::endl;
        return 1;
    }
    int leading_zeros = std::stoi(argv[1]);
    mine_block(leading_zeros);
    return 0;
}
