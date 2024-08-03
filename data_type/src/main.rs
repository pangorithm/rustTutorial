// 런타임에 판단해 빌림을 반환하는 케이스
// x, y, 반환 타입 모두 소멸 시점이 명확히 드러나지 않아 컴파일 오류 발생
// error[E0106]: missing lifetime specifier
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");

    let result = longest(&s1, &s2);
    println!("{}와 {} 중 더 긴 문자열은 '{}'", s1, s2, result); // Hello와 Rust 중 더 긴 문자열은 'Hello'
}
