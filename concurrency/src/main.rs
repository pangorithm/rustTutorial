use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // 새로운 스레드를 생성하고 그 핸들을 받습니다.
        let file = File::open("file.txt").unwrap(); // "file.txt" 파일을 엽니다.
        let reader = BufReader::new(file); // 버퍼링을 사용해 파일을 읽습니다.
        for line in reader.lines() {
            // 파일의 각 줄을 순회합니다.
            let txt = line.unwrap(); // 줄을 텍스트로 읽습니다.
            println!("{}", txt);
        }
    });

    match handle.join() {
        // join의 결과를 확인해 예외처리
        Ok(_) => {}
        Err(e) => {
            println!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
        }
    };
}
