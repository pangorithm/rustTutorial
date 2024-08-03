use std::{cell::RefCell, rc::Rc};

// NodeType을 Option<Rc<RefCell<Node>>>로 정의해, Node의 이전 노드와 다음 노드를 참조하는 타입을 나타낸다.
// Rc와 RefCell을 사용하면 노드를 가변으로 공유할 수 있으며, None을 사용하면 마지막 노드를 표현할 수 있다.
type NodeType = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32,
    prev: NodeType, // 이전 노드를 가리키는 옵셔널 포인터. 첫 노드의 경우 None
    next: NodeType, // 다음 노드를 가리키는 옵셔널 포인터. 마지막 노드의 경우 None
}

impl Node {
    fn new(item: i32) -> Self {
        Self {
            item,
            prev: None,
            next: None,
        }
    }
}

pub struct DoubleLinkedList {
    head: NodeType,
    tail: NodeType,
}

impl DoubleLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, item: i32) {
        // 새로운 노드를 생성, prev와 next는 초기에 None이다.
        let node = Rc::new(RefCell::new(Node::new(item)));

        // tail이 있는지 확인. tail이 있다면 새로운 노드를 리스트의 끝에 삽입한다.
        if let Some(tail) = self.tail.take() {
            tail.borrow_mut().next = Some(Rc::clone(&node)); // 현재 tail의 next를 새 노드로 설정
            node.borrow_mut().prev = Some(tail); // 새 노드의 prev를 현재의 tail로 설정
            self.tail = Some(node); // 새 노드를 tail로 설정
        } else {
            // tail이 없다면 리스트가 비어 있으므로 head와 tail을 새 노드로 설정
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    fn push_front(&mut self, item: i32) {
        // 새로운 노드를 생성, prev와 next는 초기에 None이다.
        let node = Rc::new(RefCell::new(Node::new(item)));

        // head가 있다면 새로운 노드를 리스트의 맨 앞에 삽입한다.
        if let Some(head) = self.head.take() {
            head.borrow_mut().prev = Some(Rc::clone(&node)); // 현재 tail의 next를 새 노드로 설정
            node.borrow_mut().next = Some(head); // 새 노드의 prev를 현재의 tail로 설정
            self.head = Some(node); // 새 노드를 tail로 설정
        } else {
            // head가 없다면 리스트가 비어 있으므로 head와 tail을 새 노드로 설정
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
        }
    }

    fn print_all(&mut self) {
        let mut currunt = match &self.head {
            Some(n) => n.clone(),
            None => {
                return;
            }
        };

        // 전체를 순회하면서 값을 출력
        loop {
            let t = currunt.clone();
            println!(
                "item: {}, RefCount: {}",
                t.borrow().item,
                Rc::strong_count(&t)
            );
            currunt = match &(t.borrow().next) {
                Some(n) => n.clone(),
                None => {
                    break;
                }
            }
            .clone();
        }
    }
}

fn main() {
    let mut list = DoubleLinkedList::new();

    println!("뒤에 1,2,3 삽입");
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    list.print_all();

    println!("맨 앞에 0 추가");
    list.push_front(0);
    list.print_all();
}
