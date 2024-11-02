use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now();
    let after = now + Duration::from_secs(3);

    println!("현재시간: {:?}", now);
    println!("+3초: {:?}", after);
    // 현재시간: SystemTime { tv_sec: 1730547893, tv_nsec: 96390848 }
    // +3초: SystemTime { tv_sec: 1730547896, tv_nsec: 96390848 }
}
