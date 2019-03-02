use std::f32;
use openssl::symm::{Cipher};
use openssl::symm::{Mode, Crypter};


pub fn fixed_xor(first: &[u8], second: &[u8]) -> Vec<u8> {
    first.iter().zip(second).map(|(&a, &b)| a ^ b).collect()
}

pub fn single_xor(first: &[u8], key: u8) -> Vec<u8> {
    first.iter().map(|&a| a ^ key).collect()
}

fn get_frequency() -> Vec<(char, f32)> {
    vec![
        ('a', 0.08167),
        ('b', 0.01492),
        ('c', 0.02782),
        ('d', 0.04253),
        ('e', 0.12702),
        ('f', 0.02228),
        ('g', 0.02015),
        ('h', 0.06094),
        ('i', 0.06966),
        ('j', 0.00153),
        ('k', 0.00772),
        ('l', 0.04025),
        ('m', 0.02406),
        ('n', 0.06749),
        ('o', 0.07507),
        ('p', 0.01929),
        ('q', 0.00095),
        ('r', 0.05987),
        ('s', 0.06327),
        ('t', 0.09056),
        ('u', 0.02758),
        ('v', 0.00978),
        ('w', 0.02360),
        ('x', 0.00150),
        ('y', 0.01974),
        ('z', 0.00074),
        (' ', 0.19182),
    ]
}

use std::collections::HashMap;

pub fn chi_squared(input: &[u8]) -> f32 {
    let mut count = HashMap::new();
    let frequencies = get_frequency();
    let mut ignored = 0;
    for c in input {
        let c = *c; //TODO: how to deref in loop?
        if c >= 65 && c <= 90 {
            *count.entry((c as char).to_ascii_lowercase()).or_insert(0) += 1;
        } else if (c >= 97 && c <= 122) || c == 32 {
            *count.entry(c as char).or_insert(0) += 1;
        } else if c >= 33 && c <= 126 {
            ignored += 1;
        } else if c == 9 || c == 10 || c == 13 {
            ignored += 1;
        } else {
            return f32::MAX;
        }
    }

    let mut chi_squared = 0f32;

    for (c, freq) in frequencies {
        let observed = match count.get(&c) {
            Some(i) => *i as f32,
            None => 0 as f32,
        };

        let len: f32 = input.len() as f32 - ignored as f32;
        let expected = len * freq;
        let diff = observed - expected;
        chi_squared += diff * diff / expected;
    }

    chi_squared + ignored as f32 * 0.7f32
}

pub fn break_single_xor(input: &[u8]) -> (Vec<u8>, u8) {
    let mut potential: Vec<(f32, u8)> = Vec::new();
    for potential_key in 0..255 {
        let decoded = single_xor(input, potential_key);
        let score = chi_squared(&decoded);
        potential.push((score, potential_key));
    }

    potential.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    let (_, key) = potential.first().unwrap();

    (single_xor(input, *key), *key)
}

pub fn repeating_xor(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    plaintext
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn hamming_distance(first: &[u8], second: &[u8]) -> u32 {
    if first.len() != second.len() {
        panic!("Hamming distance require strings with equal size");
    }

    first
        .iter()
        .zip(second)
        .map(|(a, b)| (a ^ b).count_ones())
        .sum()
}

pub fn pkcs7_pad(input: &[u8], block_length: u8) -> Vec<u8> {
    let remainder = (input.len() % block_length as usize) as u8;
    let needed_padding  = if remainder > 0 {
        block_length - remainder
    } else {
        0
    };

    let mut result = input.to_vec();
    let new_size: usize = (input.len() as u32 + needed_padding as u32) as usize;
    result.resize(new_size, needed_padding as u8);
    result
}

pub fn pkcs7_pad_remove(input: &mut Vec<u8>) {
    let pad = *input.last().unwrap() as usize;

    input.truncate(input.len() - pad);
}

pub fn aes_cbc_encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let padded = pkcs7_pad(&plaintext, 16);
    let chunks: Vec<&[u8]> = padded.chunks(16).collect();
    let mut encrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Encrypt,
        key,
        None).unwrap();
    encrypter.pad(false);

    let mut encrypted = vec![0u8; padded.len() + 16];
    let mut inside_iv = Vec::from(iv);
    let mut count = 0;
    for chunk in chunks {
        let xored = fixed_xor(chunk, &inside_iv);
        count += encrypter.update(&xored, &mut encrypted[count..]).unwrap();
        inside_iv = encrypted[count-16..count].to_vec();
    }
    encrypted.truncate(count);
    encrypted
}

pub fn aes_cbc_decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let chunks: Vec<&[u8]> = ciphertext.chunks(16).collect();
    let mut decrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None).unwrap();
    decrypter.pad(false);

    let mut decrypted = Vec::new();
    let mut inside_iv = Vec::from(iv);
    let mut _count = 0; //TODO: make fixed_xor inplace and change it to use one vec for plaintext
    for chunk in chunks {
        let mut decrypted_block = vec![0; chunk.len() + 16];
        _count += decrypter.update(chunk, &mut decrypted_block).unwrap();
        decrypted.append(& mut fixed_xor(&decrypted_block, &inside_iv));
        inside_iv = Vec::from(chunk);
    }
    pkcs7_pad_remove(&mut decrypted);

    decrypted
}
