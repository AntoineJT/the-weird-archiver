use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    println!("Hello, world!");
}

fn read_whole_file(filename: &str) -> Vec<u8> {
    let mut input = File::open(filename)
        .expect(format!("No {} file found!", filename).as_str());
    let mut buffer: Vec<u8> = Vec::new();
    input.read_to_end(&mut buffer);
    buffer
}

fn generate_batch(filename: &str) {
    let input = read_whole_file(filename);
    let mut batch = String::new();
}

fn batch_echo_cmd(filename: &str, content: &str, ext: &str) -> String {
    format!("ECHO {} > {}.{}", content, filename, ext)
}
