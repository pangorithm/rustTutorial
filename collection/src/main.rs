use std::collections::HashSet;

fn main() {
    let mut book: HashSet<String> = HashSet::new(); // String 타입의 값을 가지는 빈 HashSet 생성

    book.insert(String::from("Rust"));
    book.insert(String::from("Java"));
    book.insert(String::from("Python"));
    for data in &book {
        println!("data: {:?}", data);
    }
    // 조회 순서는 보장되지 않는다
    // # cargo run
    // data: "Java"
    // data: "Python"
    // data: "Rust"
    // # cargo run
    // data: "Rust"
    // data: "Java"
    // data: "Python"
    // # cargo run
    // data: "Python"
    // data: "Rust"
    // data: "Java"

    // 값이 있는지 확인하기
    if book.contains("JavaScript") == false {
        println!("JavaScript가 없습니다.");
        // JavaScript가 없습니다.
    }
}
