struct Node {
    value: i32,              // 노드의 값
    next: Option<Box<Node>>, // Option을 사용해 다음 노드가 없는 상황을 처리
}

impl Node {
    fn append(&mut self, elem: i32) {
        match self.next {
            Some(ref mut next) => {
                // 값이 있는 경우
                next.append(elem); // 마지막 노드를 찾기 위해 다음 노드로 이동
            }
            None => {
                // 값이 없을 경우 마지막 노드로 간주
                let node = Node {
                    // 마지막 노드에 값을 삽입
                    value: elem,
                    next: None,
                };
                self.next = Some(Box::new(node))
            }
        }
    }

    fn list(&self) {
        print!("{},", self.value);
        match self.next {
            Some(ref next) => next.list(), // 다음 노드로 이동
            None => {}
        }
    }
}

fn main() {
    let mut head = Node {
        value: 1,
        next: None,
    };

    for i in 2..10 {
        head.append(i);
    }

    head.list();
}
