fn main() {
    let condition = true;

    // if 표현식
    if condition == true {
        println!("조건이 참입니다.");
    } else {
        println!("조건이 거짓입니다.");
    }

    // let if 표현식
    let ret = if condition == true {
        String::from("조건이 참입니다.") // ; 를 붙이면 컴파일 오류가 발생한다.
    } else {
        String::from("조건이 거짓입니다.") // ; 를 붙이면 컴파일 오류가 발생한다.
    }; // 여기에 ; 를 붙여야 한다
    println!("ret={}", ret);

    let var = 1;
    // match 표현식
    match var {
        // var 값을 사용해 분기합니다.
        1 => println!("하나"),
        2 => println!("둘"),
        _ => println!("기타"),
    }

    // let match 표현식
    let ret = match var {
        // match의 결과를 ret에 저장한다.
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    }; // ; 를 붙여야 한다
    println!("ret={}", ret);
}
