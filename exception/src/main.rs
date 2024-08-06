// I/O 작업과 관련된 트레잇과 타입을 사용하기 위해 std::io를 가져온다.
// 파일 작업을 위해 std::fs::File을 가져온다.
use std::fs::File;
use std::io;
use std::io::Read;

// 파일에서 문자열을 읽어 반환하는 함수
// 파일 열기 또는 읽기 중 오류가 발생하면 오류를 반환한다.
fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e), // 파일 열기 실패 시 오류 반환
    };

    match f.read_to_string(&mut s) {
        Ok(_len) => return Ok(s), // 파일 읽기 성공 시 내용 반환
        Err(e) => return Err(e),  // 파일 읽기 실패 시 오류 반환
    };
}

fn main() {
    // 실패 시 사용자 정의 메시지와 함께 프로그램 종료
    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생했습니다.");
    // 파일 읽기 중 오류가 발생했습니다.: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("test.txt: {}", ret);
}
