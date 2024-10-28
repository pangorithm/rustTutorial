#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl AsRef<str> for Person {
    // Person의 name을 str 형태로 참조 할 수 있다.
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn greet_person<P: AsRef<str>>(person: P) {
    println!("안녕! {}", person.as_ref());
}

fn main() {
    let person = Person {
        name: String::from("루나"),
        age: 30,
    };

    // Person 구조체에 AsRef<str>을 구현했기 때문에,
    // greet_person 함수는 Person을 인자로 받아 사용할 수 있다
    greet_person(person);

    // String과 &str도 greet_person 함수 호출이 가능하다.
    greet_person(String::from("러스트"));
    greet_person("하이!");
}
