#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    cloned: bool,
}

// Person을 복제한다. Copy과는 다르게 값을 직접 설정할 수 있다
impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
            cloned: true,
        }
    }
}

fn main() {
    let person1 = Person {
        name: String::from("루나"),
        age: 10,
        cloned: false,
    };

    // person1을 복제합니다. 소유권을 잃지 않습니다.
    let person2 = person1.clone();

    println!("{:?}", person1);
    println!("{:?}", person2);
}
