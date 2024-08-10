use std::collections::LinkedList;

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new(); // i32 타입의 빈 연결 리스트 생성
    for i in 0..10 {
        list.push_back(i);
    }

    for d in list.iter_mut() {
        // 수정 가능한 반복자를 얻는다.
        *d += 10;
    }

    for d in list.iter() {
        print! {"{:?}, ", d};
    }
    // 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
}
