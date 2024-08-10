use std::collections::HashMap;

fn main() {
    let mut books: HashMap<i32, String> = HashMap::new(); // i32 타입의 key와 String 타입의 값을 가지는 빈 HashMap 생성

    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    for (key, value) in &books {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
    // # cargo run
    //    Compiling collection v0.1.0 (/root/git/rustTutorial/collection)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
    //      Running `target/debug/collection`
    // Key: 10, Value: "Rust"
    // Key: 30, Value: "Python"
    // Key: 20, Value: "Java"
    // # cargo run
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
    //      Running `target/debug/collection`
    // Key: 30, Value: "Python"
    // Key: 10, Value: "Rust"
    // Key: 20, Value: "Java"
    // # cargo run
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
    //      Running `target/debug/collection`
    // Key: 20, Value: "Java"
    // Key: 30, Value: "Python"
    // Key: 10, Value: "Rust"

    // 조회 순서는 보장되지 않는다.

    let rust = books.get(&10);
    println!("key 10은 {:?}", rust);
    // key 10은 Some("Rust")
}
