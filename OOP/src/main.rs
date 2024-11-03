// Hello라는 트레잇을 정의한다. 이 트레잇은 hello_msg 메서드를 가져야 한다.
trait Hello {
    fn hello_msg(&self) -> String; // hello_msg 메서드는 String을 반환해야 하며, 이를 구현하는 타입에서 정의해야 한다.
}

// say_hello 함수는 Hello 트레잇을 구현하는 어떠한 타입의 참조도 받을 수 있다.
// 이 함수는 전달받은 타입의 hello_msg 메서드를 호출하고 그 결과를 출력한다.
fn say_hello(say: &dyn Hello) {
    println!("{}", say.hello_msg());
}

struct Student {}

impl Hello for Student {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요! 선생님,")
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요! 오늘 수업은...")
    }
}

fn main() {
    let student = Student {};
    let teacher = Teacher {};

    say_hello(&student);
    // 안녕하세요! 선생님,
    say_hello(&teacher);
    // 안녕하세요! 오늘 수업은...
}
