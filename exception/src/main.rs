use std::fs::File;

fn main() {
    // "test.txt" 파일을 열려고 시도
    let result = File::open("test.txt");

    // result는 Result 타입이므로, 이를 통해 파일 열기의 성공 또는 실패를 확인할 수 있습니다.
    let f = match result {
        Ok(f) => f, // 파일 열기에 성공하면 File 인스턴스를 반환합니다.
        Err(err) => {
            // 파일 열기에 실패하면 에러 정보를 출력하고 프로그램을 종료합니다.
            panic!("파일 열기 실패: {:?}", err) // 파일 열기 실패: Os { code: 2, kind: NotFound, message: "No such file or directory" }
        }
    };

    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("파일 열기 성공!");
}
