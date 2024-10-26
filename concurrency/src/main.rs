use std::sync::Mutex;
use std::thread;

static counter: Mutex<i32> = Mutex::new(0); // conuter를 전역 변수로 정의

fn inc_counter() {
    let mut num = counter.lock().unwrap();
    *num = *num + 1;
} // inc_counter를 벗어나는 순간 counter는 unlock됩니다.

fn main() {
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let th = thread::spawn(inc_counter); // counter를 증가합니다.
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap()); // counter 값을 획득합니다.
}
