use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() { // subtle behavior: BufReader::lines() removes trailing newlines
        let line = line_.unwrap(); // unwraps the Result but at the risk of crashing if an error occurs
        println!("{} ({} bytes long)", line, line.len());
    }
}
