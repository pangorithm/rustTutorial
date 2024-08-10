use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new(); // i32 타입의 빈 연결 리스트 생성
    list.push_back(1); // 리스트의 뒤에 숫자 1을 추가
    list.push_back(2); // 리스트의 뒤에 숫자 2을 추가
    list.push_back(3); // 리스트의 뒤에 숫자 3을 추가

    for i in &list {
        print!("{}, ", i);
    }
    // 1, 2, 3,
    println!("");

    list = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

    // 9번째 인덱스 찾기
    let idx = 9;
    let mut i = 0;
    let mut target: i32 = 0;

    for data in &list {
        if i == idx {
            target = *data;
        }

        i += 1;
    }
    println!("target: {:?}", target);
    // target: 9

    println!("iterator target: {:?}", list.iter().nth(9));
    // iterator target: Some(9)
}
