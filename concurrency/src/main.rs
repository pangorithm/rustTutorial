use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let lock_a = Arc::new(Mutex::new(0));
    let lock_b = Arc::new(Mutex::new(0));

    let lock_a_ref = lock_a.clone();
    let lock_b_ref = lock_a.clone();

    let thread1 = thread::spawn(move || {
        // 강제로 교착상태를 만든다.
        let a = lock_a.lock().unwrap();
        let b = lock_b_ref.lock().unwrap(); // lock_b는 thread2에 의해 잠겨있는 상태
    });

    let thread2 = thread::spawn(move || {
        let a = lock_b.lock().unwrap();
        let b = lock_a_ref.lock().unwrap(); // lock_a는 thread1에 의해 잠겨있는 상태
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("프로그램 종료");
}
