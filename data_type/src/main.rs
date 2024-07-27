fn main() {
    let arr1 = [1, 2, 3, 4, 5];

    for a in arr1 {
        println!("{}", a);
    }
    println!("");

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    for i in 0..arr2.len() {
        println!("{}", arr2[i]);
    }
    println!("");

    std::env::set_var("RUST_BACKTRACE", "1"); // RUST_BACKTRACE 활성화. cargo run으로 실행 시, RUST_BACKTRACE=1 cargo run으로 실행한 것과 같다.
    use std::io;
    let arr3: [i32; 5] = [1, 2, 3, 4, 5];
    println!("숫자를 입력해주세요.");
    let mut read = String::new();
    io::stdin().read_line(&mut read).unwrap();
    let index: i32 = read.trim().parse().unwrap();
    println!("arr3[{}]={}", index, arr3[index as usize]);
}
