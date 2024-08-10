fn main() {
    let vec = vec![1, 2, 3];
    for item in vec.into_iter() {
        // vec의 소유권은 이동했으므로 이후에 vec을 사용할 수 없음
        println!("{}", item);
    }

    // println!("{:?}", vec);
    // 8   |     println!("{:?}", vec);
    //     |                      ^^^ value borrowed here after move
}
