fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0으로 나눌 수 없습니다.")
        // thread 'main' panicked at src/main.rs:3:9:
        // 0으로 나눌 수 없습니다.
        // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    }

    a / b
}

fn main() {
    let ret = div(1, 0); // 1 나누기 0을 시도해 오류 발생
                         // thread 'main' panicked at src/main.rs:2:5:
                         // attempt to divide by zero
                         // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    println!("{}", ret);
}
