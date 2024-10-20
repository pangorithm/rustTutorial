use tokio::time;
use std::time::Duration;

async fn sleep_10sec() {
    for i in 1..10 {
        println!(".");
        time::sleep(Duration::from_millis(1000)).await; // 1초간 10회 대기
    }
}

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        println!("{} ", i);
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let f1 = sleep_10sec();
    let f2 = calc_sum(1, 10);
    let (_, sum) = tokio::join!(f1, f2); // f1과 f2가 끝나기를 기다린다.

    sum
}

#[tokio::main]
async fn main() {
    let sum = calc().await;
    println!("1부터 10까지의 합: {}", sum);
}
