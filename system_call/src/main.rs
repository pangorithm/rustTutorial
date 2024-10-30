use std::process;
use std::process::Command;

fn main() {
    // 쉘에서 echo를 실행
    let echo = Command::new("echo")
        .arg("echo 실행") // "echo 실행"이라는 인자를 추가한다.
        .output()
        .expect("echo 실행 실패");

    // 명령어의 출력을 UTF-8 문자열로 변환
    let ret = String::from_utf8_lossy(&echo.stdout);

    println!("결과: {}", ret);

    let pid = process::id(); // 현재 실행중인 프로세스의 pid 획득
    println!("Process ID: {}", pid);
}
