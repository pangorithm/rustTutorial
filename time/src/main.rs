use chrono::offset::TimeZone;
use chrono::{FixedOffset, Local, Utc};

fn main() {
    // UTC 시간 획득
    let now_utc = Utc::now();
    println!("Utc 시각: {}", now_utc);

    // 로컬 시간
    let now_local = Local::now();
    println!("Local 시각: {}", now_local);

    // 서울 시간 획득 UTC+9
    let seoul_offset = FixedOffset::east(9 * 3600); // +9
    let seoul = seoul_offset.from_utc_datetime(&now_utc.naive_utc());
    println!("한국 시각: {}", seoul);
}
// Utc 시각: 2024-11-02 11:56:14.679871697 UTC
// Local 시각: 2024-11-02 20:56:14.679901283 +09:00
// 한국 시각: 2024-11-02 20:56:14.679871697 +09:00
