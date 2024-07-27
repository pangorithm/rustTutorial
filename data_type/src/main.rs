use std::io;

fn main() {
    println!("학번을 입력하세요");
    let mut id = String::new();
    io::stdin().read_line(&mut id);
    let id: i32 = id.trim().parse().unwrap();

    println!("이름을 입력해주세요.");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let name = name.trim().to_string(); // 공백 제거

    println!("이메일을 입력해주세요.");
    let mut email = String::new();
    io::stdin().read_line(&mut email);
    let email = email.trim().to_string(); // 공백 제거

    let stu = create_student(id, name, email); // 학생 인스턴스 생성
    println!("학번={}, 이름={}, 이메일={}", stu.id, stu.name, stu.email); // 인스턴스의 내부값 참조
    println!("stu={:?}", stu); // :? 는 Debug 포맷을 기본 출력 포맷으로 지정한다.
}

#[derive(Debug)] // 이 어노테이션은 구조체의 디버그 포메터를 지정해준다. 없으면 println!("stu={:?}", stu);에서 에러가 발생한다.
struct Student {
    id: i32,
    name: String,
    email: String,
}

fn create_student(id: i32, name: String, email: String) -> Student {
    Student {
        // Student 인스턴스 생성
        id: id,
        name: name,
        email: email,
    } // return 값은 ; 이 없다.
}
