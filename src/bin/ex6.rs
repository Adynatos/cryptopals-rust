use cryptopals::break_single_xor;
use cryptopals::hamming_distance;
use cryptopals::repeating_xor;

extern crate base64;
extern crate hex;

use std::fs;
use std::str;

fn main() {
    let first = "this is a test";
    let second = "wokka wokka!!!";
    assert_eq!(37, hamming_distance(first.as_bytes(), second.as_bytes()));

    let encrypted = fs::read_to_string("resources/6.txt").expect("Failed to read input file");
    let encrypted = encrypted.replace("\n", "");
    let encrypted = base64::decode(&encrypted).expect("Error during base64 decoding");;


    let mut potential_keysizes = Vec::new();
    for keysize in 2..40 {
        let mut sum = 0f32;
        let chunks: Vec<&[u8]> = encrypted.chunks(keysize).take(4).collect();

        for (idx, chunk) in chunks.iter().enumerate() {
            //TODO: avoid?
            for second_chunk in chunks.iter().skip(idx + 1) {
                sum += hamming_distance(chunk, second_chunk) as f32;
            }
        }
        sum = sum / keysize as f32; //normalized by size
        potential_keysizes.push((keysize, sum));
    }

    potential_keysizes.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let (keysize, score) = potential_keysizes.first().unwrap();
    println!("Found keysize: {}, {}", keysize, score);

    let mut transposed: Vec<Vec<u8>> = Vec::new();
    for _ in 0..*keysize {
        transposed.push(Vec::new()); //TODO: find smarter way to create this vec
    }

    for block in encrypted.chunks(*keysize) {
        for (idx, val) in block.iter().enumerate() {
            transposed[idx].push(*val);
        }
    }

    let mut key: Vec<u8> = Vec::new();
    for block in transposed {
        let (_, key_part) = break_single_xor(block.as_slice());
        key.push(key_part);
    }

    println!(
        "Found key: {}",
        str::from_utf8(&key).expect("Failed to parse key")
    );

    let decrypted = repeating_xor(&encrypted, &key);
    let decrypted = str::from_utf8(&decrypted).expect("Failed to parse decrypted text");
    println!("Decrypted msg: {}", decrypted);
}
