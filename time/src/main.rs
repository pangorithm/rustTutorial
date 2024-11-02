use chrono::Local;

fn main() {
    // 현재 로컬 날짜와 시간을 가져온다.
    let now = Local::now();

    println!("{}", now.format("%Y-%m-%d")); // YYYY-MM-DD

    println!("{}", now.format("%H:%M:%S")); // HH:MM:SS

    println!(
        "{}",
        now.format("오늘은 %A, %B, %d, %y. 현재 시간은%Y-%m-%d")
    );
}
// 2024-11-02
// 20:49:43
// 오늘은 Saturday, November, 02, 24. 현재 시간은2024-11-02
