use std::cell::RefCell;
use std::rc::Rc;

struct Person {
    id: i32,
    next: RefCell<Option<Rc<Person>>>,
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

    let mut p3 = Rc::new(Person {
        id: 3,
        next: RefCell::new(None),
    });

    // 순환참조 발생
    let mut next = p1.next.borrow_mut();
    *next = Some(p2.clone());

    let mut next = p2.next.borrow_mut();
    *next = Some(p1.clone());

    println!(
        "p1 RefCount: {}, p2 RefCount: {}",
        Rc::strong_count(&p1),
        Rc::strong_count(&p2)
    );
    // p1 RefCount:2, p2 RefCount:2
    // p3 Drop!
}
