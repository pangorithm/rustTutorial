use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let pt = Point { x: 10, y: 20 };
    let json = serde_json::to_string(&pt).unwrap(); // pt를 json으로 변환
    println!("json: {}", json);

    let pt: Point = serde_json::from_str(&json).unwrap(); // json을 사용해 Point를 생성
    println!("point: [{}, {}]", pt.x, pt.y);
}
