fn main() {
    // 클로저 기초 예제
    let mut x = 5;
    let mut inc = || {
        x += 1;
    }; // 할당문이므로 ; 이 필요하다
    inc();
    println!("변수 x: {}", x);

    // 클로저에 파라미터 정의
    let x = 10;
    let add = |y| x + y; // add는 클로저 함수가 된다.
    println!("10+5={}", add(5));

    // move 키워드를 사용해 클로저의 소유권 이전
    let s = String::from("Hello");
    let f = move || {
        // move 클로저는 소유권을 이전한다.
        println!("s: {}", s); // 여기서 s의 소유권을 가져간다.
    }; // 할당문이므로 ; 이 필요하다

    f();
    //println!("s: {}", s); // 컴파일 오류: s의 소유권이 없습니다.
}
