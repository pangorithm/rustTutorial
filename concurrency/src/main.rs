use std::sync::mpsc;
use std::thread;

fn main() {
    // mpsc 채널을 생성합니다. tx는 송신자, rx는 수신자입니다.
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1); // tx1 복제

    // 1부터 50까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..51 {
            sum = sum + i;
        }

        // 계산된 합을 채널로 보냅니다.
        tx1.send(sum).unwrap();
    });

    // 51부터 100까지의 합
    thread::spawn(move || {
        let mut sum = 0;

        for i in 51..101 {
            sum = sum + i;
        }

        // 계산된 합을 채널로 보냅니다.
        tx2.send(sum).unwrap();
    });

    let mut sum = 0;
    for val in rx {
        println!("수신: {}", val);
        sum = sum + val;
    }
    println!("1부터 100까지의 합: {}", sum);
}
