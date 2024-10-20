use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc 채널을 생성합니다. tx는 송신자, rx는 수신자입니다.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..101 {
            sum = sum + i;
        }

        // 계산된 합을 채널로 보냅니다.
        tx.send(sum).unwrap();
    });

    // 채널에서 메시지를 수신합니다.
    let sum = rx.recv().unwrap();
    println!("1부터 100까지의 합: {}", sum);
}
