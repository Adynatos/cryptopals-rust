use std::fs;

fn main() {
    let encrypted = fs::read_to_string("resources/8.txt").expect("Failed to read input file");
    for (line_number, line) in encrypted.split_whitespace().enumerate() {
        let input = base64::decode(&line).expect("Failed to decode base64");
        let chunks: Vec<&[u8]> = input.chunks(16).collect();
        for (idx, chunk) in chunks.iter().enumerate() {
            for another_chunk in chunks.iter().skip(idx + 1) {
                if chunk == another_chunk {
                    println!("Line nr: {} was encrypted with ECB mode", line_number);
                    return;
                }
            }
        }
    }
}
