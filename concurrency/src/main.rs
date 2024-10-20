use futures::executor::block_on;

// async 키워드를 사용해 비동기 함수를 정의합니다.
async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        sum += i;
    }

    sum
}

fn main() {
    let future = calc_sum(1, 100);

    let sum = block_on(future);
    println!("1부터 100까지의 합: {}", sum);
}
