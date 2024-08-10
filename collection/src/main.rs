fn main() {
    let str = String::from("안녕");
    let idx = 123;
    let s = format!("{} {}", str, idx); // str과 idx를 결함한다.
    println!("{}", s);
    // 안녕 123
}
