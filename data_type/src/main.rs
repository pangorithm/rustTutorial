fn main() {
    let var = 1;
    println!("var={}", var);
    let var = var + 1; // 기존의 var 변수는 소멸되며 새로운 var 변수가 생성
    println!("var={}", var);
    let var = String::from("기존 var를 섀도잉");
    println!("var={}", var);
}
