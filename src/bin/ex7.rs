use openssl::symm::{decrypt, Cipher};
use std::fs;
use std::str;

fn main() {
    let encrypted = fs::read_to_string("resources/7.txt").expect("Failed to read input file");
    let encrypted = encrypted.replace("\n", "");
    let encrypted = base64::decode(&encrypted).expect("Failed to decode base64");

    let key = "YELLOW SUBMARINE";

    let cipher = Cipher::aes_128_ecb();

    let decrypted = decrypt(cipher, key.as_bytes(), None, &encrypted).unwrap();
    let msg = str::from_utf8(&decrypted).expect("Failed to parse decrypted text");

    println!("{}", msg);
}
