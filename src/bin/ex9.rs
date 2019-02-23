use cryptopals::pkcs7_pad;

fn main() {
    let input = "YELLOW SUBMARINE";
    let block_length = 20;

    assert_eq!(pkcs7_pad(input.as_bytes(), block_length), "YELLOW SUBMARINE\x04\x04\x04\x04".as_bytes());
}
