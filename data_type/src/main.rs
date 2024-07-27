fn main() {
    // 문자열 리터럴을 사용
    let s: &str = "Hello 러스트!"; // 메모리에 저장된 문자열의 주소를 참조로 사용하기 때문에 항상 &str 형태로 사용된다.

    println!("문자열: {}", s);

    // 문자열 슬라이싱
    let slice: &str = &s[0..5];
    println!("슬라이스: {}", slice);

    // 문자열 변환
    let str: &str = "  Hello Rust  ";
    println!("{}", str.trim());
    println!("{}", str.to_lowercase());
    println!("{}", str.to_uppercase());
}
