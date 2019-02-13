extern crate hex;

use cryptopals::break_single_xor;
use std::str;

fn main() {
    let encrypted =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
            .unwrap();
    let (decrypted, key) = break_single_xor(&encrypted);

    println!(
        "Decrypted: {}, with key: [{}]",
        str::from_utf8(&decrypted).expect("Failed to parse decrypted text"),
        key as char
    );
}
