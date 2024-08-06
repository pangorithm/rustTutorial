fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let ret = div(1, 0); // 1 나누기 0을 시도해 오류 발생
                         // thread 'main' panicked at src/main.rs:2:5:
                         // attempt to divide by zero
                         // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    println!("{}", ret);
}
