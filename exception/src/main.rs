use std::fs::File;

fn main() {
    // unwrap 메서드는 Result가 Ok 값이면 그 값을 반환하고, Err 값이면 패닉을 일으킨다.
    let f = File::open("test.txt").unwrap();
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("파일 열기 성공!");
}
