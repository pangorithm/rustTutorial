use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file); // BufReader를 생성합니다.

    for line in reader.lines() {
        // file을 읽습니다.
        let line = line.unwrap();
        println!("{}", line);
    }
}
