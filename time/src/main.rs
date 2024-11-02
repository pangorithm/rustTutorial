use std::time::SystemTime;

fn main() {
    // 현재 시스템 시간을 가져온다.
    let now = SystemTime::now();

    // 현재 시스템 시간을 디버그 형식으로 출력한다.
    println!("{:?}", now);
    // SystemTime { tv_sec: 1730547487, tv_nsec: 510123098 }
}
