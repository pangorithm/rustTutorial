fn main() {
    let tuple = (1, 2, 3);
    println!("tuple: {}, {}, {}", tuple.0, tuple.1, tuple.2);

    let (x, y, z): (i32, char, bool) = (1, 'a', true);
    println!("x={}, y={}, z={}", x, y, z);
}
