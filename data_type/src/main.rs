use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Person {
    id: i32,
    next: RefCell<Option<Weak<Person>>>,
}

// 참조 횟수가 0이 되는 순간 호출됨
impl Drop for Person {
    fn drop(&mut self) {
        println!("p{} Drop!", self.id);
    }
}

fn main() {
    let mut p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),
    });

    let mut p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
    });

    println!(
        "p1 RefCount: {}, p2 RefCount: {}",
        Rc::strong_count(&p1),
        Rc::strong_count(&p2)
    );
    // p1 RefCount:1, p2 RefCount:1

    // 순환참조 발생
    let mut next = p1.next.borrow_mut();
    *next = Some(Rc::downgrade(&p2)); // Rc 타입의 참조를 Weak 타입으로 변환한다.

    let mut next = p2.next.borrow_mut();
    *next = Some(Rc::downgrade(&p2)); // Rc 타입의 참조를 Weak 타입으로 변환한다.

    println!(
        "p1 RefCount: {}, p2 RefCount: {}",
        Rc::strong_count(&p1),
        Rc::strong_count(&p2)
    );
    // p1 RefCount:1, p2 RefCount:1
    // p2 Drop!
    // p1 Drop!
}
