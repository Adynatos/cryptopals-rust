extern crate hex;

use cryptopals::fixed_xor;
use std::str;

fn main() {
    let first = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let second = hex::decode("686974207468652062756c6c277320657965").unwrap();

    let res = fixed_xor(&first, &second);
    println!("{}", str::from_utf8(&res).unwrap());

    assert_eq!(hex::encode(&res), "746865206b696420646f6e277420706c6179");
}
