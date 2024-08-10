fn main() {
    // let mut v: Vec<i32> = Vec::new(); // 빈 i32 타입의 벡터 v를 생성한다.
    // let mut v: Vec<i32> = vec![1, 2, 3]; // 초기값을 가지는 벡터
    let mut v: Vec<i32> = vec![]; // 빈 벡터

    for i in 1..10 {
        // 1부터 9 까지의 숫자를 반복한다.
        v.push(i);
    }

    for d in &v {
        // 벡터 v의 각 요소에 대해 반복한다.
        print!("{}, ", d);
    }
    // 1, 2, 3, 4, 5, 6, 7, 8, 9,
}
