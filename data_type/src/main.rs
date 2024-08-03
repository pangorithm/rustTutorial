use std::cell::RefCell; // RefCell은 내부 가변성을 제공하며, 런타임에 빌림 규칙을 검사한다.
use std::rc::Rc;

struct Node {
    name: String,
    value: i32,
    next: RefCell<Option<Rc<Node>>>, // RefCell로 래핑되어 있기 때문에 불변 참조에서도 수정 가능
}

fn push_back(tail: Rc<Node>, name: String, value: i32) -> Rc<Node> {
    let n = Rc::new(Node {
        name: name,
        value: value,
        next: RefCell::new(None),
    });

    // tail의 next 필드에 대한 가변 참조를 얻는다.
    let mut next = tail.next.borrow_mut();
    *next = Some(n.clone()); // p1 뒤에 p2를 추가해 연결 리스트를 연결

    n // 새로 생성된 노드를 반환한다. 이제 이 노드가 리스트의 마지막 노드이다.
}

fn main() {
    let head = Rc::new(Node {
        name: String::from("Luna"),
        value: 30,
        next: RefCell::new(None),
    });

    let tail = push_back(head.clone(), String::from("Rust"), 10);
    let tail = push_back(tail, String::from("Wikibooks"), 20);

    let mut current = head.clone();

    loop {
        print!("{} -> ", current.name);
        let t = current.clone(); // 다음 노드를 참조하기 위해 현재 노드를 복제
        current = match &(*(t.next.borrow_mut())) {
            Some(n) => n,
            None => break,
        }
        .clone();
    }
}
