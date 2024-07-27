fn main() {
    let mut s = String::from("Hello"); // Hello로 String 생성
    println!("{}", s);
    s.push_str(" Rust!"); // s뒤에 Rust! 추가
    println!("{}", s);
}
