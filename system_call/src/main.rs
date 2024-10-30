use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    // 사전에 touch test.txt 파일을 만든다.
    let path = Path::new("test.txt");
    let metadata = path.metadata().unwrap();

    // 리눅스에서만 작동한다.
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    println!("파일 접근 권한: {:o}", mode);
    // -rw-r--r--  1 root root    0 Oct 30 19:55 test.txt
    // 파일 접근 권한: 100644
}
