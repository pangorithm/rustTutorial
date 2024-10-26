use tokio::fs::File;
use tokio::io::{stdin, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() {
    let mut reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    loop { // quit가 입력될 때까지 입력을 받음
        match lines.next_line().await.unwrap() {
            Some(input) => {
                println!("입력: {}", input);

                if input == "quit" { // quit을 입력받으면 종료합니다.
                    break;
                }
            }
            None => {
                break;
            }
        };

    }
}
