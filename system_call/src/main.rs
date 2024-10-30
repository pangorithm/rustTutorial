use std::fs;

fn main() -> std::io::Result<()> {
    // 현재 디렉터리의 파일 및 디렉터리 목록 읽기
    let entries = fs::read_dir(".")?;

    // 각 항목을 반복하면서 이름을 출력한다.
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        println!("{}", file_name_str);
    }

    Ok(())
}
