use std::fs;
use std::str;
use cryptopals::aes_cbc_decrypt;

fn main() {
    let encrypted = fs::read_to_string("resources/10.txt").expect("Failed to read input file");
    let encrypted = encrypted.replace("\n", "");
    let encrypted = base64::decode(&encrypted).expect("Failed to decode base64");

    let key = "YELLOW SUBMARINE";
    let iv = "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    let decrypted = aes_cbc_decrypt(&encrypted, key.as_bytes(), iv.as_bytes());
    let msg = str::from_utf8(&decrypted).expect("Failed to parse decrypted text");

    println!("{}", msg);
}
