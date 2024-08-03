use std::rc::Rc;

struct Node {
    name: String,
    value: i32,             // 노드의 값
    next: Option<Rc<Node>>, // Option을 사용해 다음 노드가 없는 상황을 처리
}

fn main() {
    let n1 = Rc::new(Node {
        name: String::from("Luna"),
        value: 30,
        next: None,
    });

    let n2 = Rc::new(Node {
        name: String::from("Rust"),
        value: 20,
        next: Some(n1.clone()), // Rc::clone을 사용해 참조 카운트를 증가시킴
    });

    print!("{} -> ", n2.name); // n2의 이름 출력

    match &n2.next {
        Some(n) => {
            println!("{}", n.name); // 다음 노드가 있다면 그 이름을 출력
        }
        None => {} // 다음 노드가 없다면 아무것도 하지 않음
    };
}
