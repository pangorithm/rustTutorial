use std::fs::File;
use std::io::Read;
use std::os::unix::io::{FromRawFd, IntoRawFd};

fn main() {
    // test.txt를 연다.
    let f = File::open("test.txt").unwrap();

    // f의 파일 서술자를 획득한다
    let fd = f.into_raw_fd();

    // 파일 서술자로부터 File 객체를 생성한다.
    let mut f = unsafe { File::from_raw_fd(fd) };

    // 파일 내용을 출력한다.
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("파일 읽기 실패");
    println!("{}", contents);
}
