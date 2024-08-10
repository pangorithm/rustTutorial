use std::collections::HashMap;

pub struct User {
    pub id: i32,
    pub age: i32,
    pub name: String,
}

pub struct UserManager {
    user_map: HashMap<i32, User>, // 사용자 ID와 사용자 정보를 매핑하는 HashMap
}

impl UserManager {
    // 새 UserManager 인스턴스 생성
    pub fn new() -> UserManager {
        let mgr = UserManager {
            user_map: HashMap::new(), // 빈 HashMap으로 초기화
        };
        mgr
    }

    // 사용자 추가 메서드
    pub fn add_user(&mut self, id: i32, age: i32, name: String) -> bool {
        let mut user = User {
            id: id,
            age: age,
            name: name,
        };

        self.user_map.entry(user.id).or_insert(user); // ID가 존재하면 갱신, 없으면 삽입
        true
    }

    // 사용자 제거 메서드
    pub fn remove_user(&mut self, id: i32) -> bool {
        if self.user_map.contains_key(&id) == false {
            // 해당 ID가 없으면 false 반환
            return false;
        }

        self.user_map.remove(&id); // 해당 ID로 사용자 제거
        true
    }

    // 특정 사용자 조회 메서드
    pub fn get_user(&mut self, id: i32) -> Option<&User> {
        self.user_map.get(&id) // ID로 사용자 정보 반환
    }

    // 모든 사용자 정보를 반환하는 메서드
    pub fn get_all(&mut self) -> Vec<&User> {
        let mut v: Vec<&User> = Vec::new();

        for u in self.user_map.values() {
            // 모든 사용자를 순회하며 벡터에 추가
            v.push(&u);
        }

        return v; // 사용자 참조를 담은 벡터 반환
    }
}
