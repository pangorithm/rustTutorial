use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // 공유될 카운터를 Arc와 Mutex로 감싸준다.
    let mut thread_vec = vec![]; // 스레드를 저장할 벡터

    for _ in 0..100 {
        let _cnt = counter.clone(); // 현재 카운터의 클론을 생성한다. Arc를 사용하면 여러 스레드 간에 안전하게 공유할 수 있다.
        let th = thread::spawn(move || {
            let mut num = _cnt.lock().unwrap(); // 뮤텍스로부터 안전하게 락을 얻어와 참조를 획득한다.
            *num = *num + 1;
        });
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap(); // 모든 스레드가 완료될 때까지 기다린다.
    }

    println!("결과: {}", *counter.lock().unwrap()); // 최종 카운터 값을 출력한다.
}
