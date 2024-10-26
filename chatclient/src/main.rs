use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut username = String::new();

    let stream = TcpStream::connect("localhost:1234").await?; // localhost의 1234번 포트로 접속
    let (reader, mut writer) = tokio::io::split(stream); // stream을 reader와 writer로 분리
    let mut reader = BufReader::new(reader);

    print!("대화명을 입력하세요: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?; // username을 입력받음
    writer.write_all(username.as_bytes()).await?; // username을 전송

    tokio::spawn(async move {
        loop { // 이벤트 루프를 구동
            let mut message = String::new();

            match reader.read_line(&mut message).await { // 다른 사람의 대화를 수신
                Ok(_) => {
                    print!("{}", message);
                },
                Err(_) => {
                    break;
                }
            };
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        writer.write_all(input.as_bytes()).await?; // 사용자의 채팅 데이터를 전송

        if input.trim() == "/quit" {
            break;
        }
    }

    Ok(())
}