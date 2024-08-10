use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Write;

use crate::business::user::User;
use crate::business::user::UserManager;

// 사용자 목록을 파일에 저장하는 함수
pub fn save(file_name: String, user_vec: Vec<&User>) -> Result<(), Error> {
    let mut buffer = File::create(file_name).expect("파일을 열 수 없습니다."); // 파일 생성
    for u in user_vec.iter() {
        let f = format!("{} {} {}\n", u.id, u.age, u.name); // 사용자 정보 포매팅
        buffer.write(f.as_str().as_bytes())?; // 파일에 쓰기
    }

    Ok(())
}

// 파일에서 사용자 목록을 불러오는 함수
pub fn load(file_name: String) -> Vec<User> {
    let mut user_vec: Vec<User> = Vec::new(); // 사용자 목록을 담을 벡터
    let txt = fs::read_to_string(file_name).expect("파일을 읽을 수 없습니다."); // 파일 읽기

    // 파일의 각 줄을 분석
    for ln in txt.split("\n") {
        if ln.len() == 0 {
            break;
        }

        let tok: Vec<&str> = ln.split(" ").collect(); // 공백으로 분리

        // 분리된 토큰을 사용해 사용자 정보 생성
        user_vec.push(User {
            id: tok[0].parse::<i32>().unwrap(),
            age: tok[1].parse::<i32>().unwrap(),
            name: tok[2].to_string(),
        });
    }

    user_vec // 사용자 벡터 반환
}
