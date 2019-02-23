use std::str;

fn main() {
    let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex::decode(hex_str).expect("Failed to convert hexs string to bytes");
    let encoded = base64::encode(&bytes);

    assert_eq!(
        encoded,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
    let decoded = base64::decode(&encoded).unwrap();
    let decoded = str::from_utf8(&decoded).unwrap();

    println!("{:?}", decoded);
}
