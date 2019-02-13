use cryptopals::break_single_xor;
use cryptopals::chi_squared;
use std::fs;
use std::str;

fn main() {
    let encrypted = fs::read_to_string("resources/4.txt").expect("Failed to read input file");

    let encrypted = encrypted.split("\n");
    let mut chi = Vec::new();

    for line in encrypted {
        let line = hex::decode(line).expect("Error during hex-decoding line");
        let (decrypted, _key) = break_single_xor(&line);
        chi.push((chi_squared(&decrypted), decrypted));
    }

    chi.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let (_, msg) = chi.first().unwrap();
    let msg = str::from_utf8(&msg).expect("Failed to parse decrypted text");
    print!("Decoded msg: {}", msg);
}
