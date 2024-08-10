fn main() {
    let v = vec![1, 2, 3]; // i32 타입의 벡터 v를 생성하고 초깃값으로 1, 2, 3을 설정한다.
    let one = v[0]; // 벡터의 첫 번째 요소를 one에 할당한다.
    let two = v.get(1); // 벡터의 두 번째 요소를 가져온다. Option 타입을 반환하므로 결과는 Some(2)이다.
    let nine = v.get(9); // 벡터 크기를 벗어나는 인덱스

    println!("One: {:?}, Two: {:?}, Nine: {:?}", one, two, nine);
    // One: 1, Two: Some(2), Nine: None

    println!("{:?}", v[9]); // panic! 발생
                            // thread 'main' panicked at src/main.rs:9:29:
                            // index out of bounds: the len is 3 but the index is 9
}
