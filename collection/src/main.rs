use std::collections::BinaryHeap;

fn main() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new(); // String 타입의 값을 가지는 빈 HashSet 생성

    heap.push(3);
    heap.push(9);
    heap.push(2);
    heap.push(5);

    while heap.is_empty() == false {
        print!("{:?}, ", heap.pop()); // 힙의 최대값을 꺼내어 출력한다. pop()은 Option<T>를 반환하므로 {:?}를 사용해 출력한다.
    }
    // Some(9), Some(5), Some(3), Some(2),
}
