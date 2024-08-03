use std::rc::Rc;

struct Node {
    name: String,
    value: i32,             // 노드의 값
    next: Option<Rc<Node>>, // Option을 사용해 다음 노드가 없는 상황을 처리
}

fn push_front(head: Rc<Node>, name: String, value: i32) -> Rc<Node> {
    let n = Rc::new(Node {
        name: name,
        value: value,
        next: Some(head.clone()),
    });

    n.clone() // 새로 생성된 노드의 Rc를 클론해 반환한다. 이제 이 노드가 새로운 head이다.
}

fn main() {
    let head = Rc::new(Node {
        name: String::from("Luna"),
        value: 30,
        next: None,
    });

    let head = push_front(head, String::from("Rust"), 10);
    let head = push_front(head, String::from("Wikibooks"), 20);

    let mut current = head.clone();

    loop {
        print!("{} -> ", current.name);
        current = match &current.next {
            Some(n) => n,
            None => break,
        }
        .clone();
    }
}
