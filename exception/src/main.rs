use std::fs::File;
use std::io;
use std::io::Read;

// 파일 열기 또는 읽기 중 오류가 발생하면 오류를 반환한다.
fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new(); // 읽은 문자열을 저장할 문자열 객체
    let mut f = File::open(path)?; // 파일 열기. 실패하면 즉시 오류 반환
    let mut _ret = f.read_to_string(&mut s)?; // 파일 읽기. 실패하면 즉시 오류 반환
    Ok(s) // 파일 읽기 성공 시 문자열 반환
}

fn main() {
    // 실패 시 사용자 정의 메시지와 함께 프로그램 종료
    let ret = read_from_file(String::from("test.txt")).expect("파일이 없습니다");
    // 파일이 없습니다: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("test.txt: {}", ret);
}
