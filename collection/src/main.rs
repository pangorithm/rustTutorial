fn main() {
    let mut eng = String::new();
    eng.push_str("hello");
    let chn = "你好".to_string();
    let kor = String::from("안녕하세요");

    println!("{}, {}, {}", eng, chn, kor);
}
// hello, 你好, 안녕하세요
