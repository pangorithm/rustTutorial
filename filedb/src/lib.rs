pub mod business;
pub mod database;

use business::user::User;
use business::user::UserManager;

#[test]
fn it_works() {
    let mut user_mgr = UserManager::new();
    user_mgr.add_user(1, 20, String::from("러스트"));
    user_mgr.add_user(2, 30, String::from("책"));

    user_mgr.save();

    let mut new_user_mgr = UserManager::new();
    new_user_mgr.load();

    let user = match new_user_mgr.get_user(1) {
        Some(u) => u,
        _ => {
            panic!("사용자를 찾을 수 없습니다.")
        }
    };

    assert_eq!(user.id, 1);
    let all_user = user_mgr.get_all();
    for u in all_user.iter() {
        println!("id: {}, age: {}, name: {}", u.id, u.age, u.name);
    }
    // # cargo test -- --nocapture
    // id: 2, age: 30, name: 책
    // id: 1, age: 20, name: 러스트
    // test it_works ... ok
}
