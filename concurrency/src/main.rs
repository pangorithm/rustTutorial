use std::thread;

fn main() {
    // 새로운 스레드를 생성하고 실행
    let handle = thread::spawn(|| {
        println!("스레드에서 실행"); // 새로 생성된 스레드에서 출력
    });

    handle.join().unwrap(); // 스레드가 완료될 때까지 대기 (종료 대기)
}
