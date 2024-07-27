fn main() {
    print_add(1, 2);

    let ret = return_add(2, 3);
    println!("2+3={}", ret);

    let x = 3;
    let y = 4;
    // 익명함수
    let ret = {
        // 익명함수의 반환값을 ret에 저장한다.
        x + y
    }; // ; 이 필요하다.

    println!("{}+{}={}", x, y, ret);
}

// 함수
fn print_add(x: i32, y: i32) {
    println!("{}+{}={}", x, y, (x + y));
}

// 값을 반환하는 함수
fn return_add(x: i32, y: i32) -> i32 {
    x + y // ; 이 없다.
}
