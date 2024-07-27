fn main() {
    // let var = 1; // 불변 변수 생성
    // var = 2; // 컴파일 오류 발생

    let mut var = 1;
    println!("수정 전={}", var);
    var = 2;
    println!("수정 후={}", var);
}
