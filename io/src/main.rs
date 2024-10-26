use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let mut file = File::open("input.txt").await.unwrap(); // 비동기 방식으로 file handel을 얻습니다.
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap(); // 비동기 방식으로 file을 읽습니다.

    println!("{}", contents);

    let mut file = File::create("output.txt").await.unwrap(); // 비동기 방식으로 file을 생성합니다.
    file.write_all(contents.as_bytes()).await.unwrap(); // 비동기 방식으로 file에 내용을 저장합니다.
}
