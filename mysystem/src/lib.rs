pub mod business;
pub mod database;
pub mod presentation;

#[test]
fn it_works() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}

pub mod my_module {
    // 이 함수는 외부에서 접근 가능합니다.
    pub fn public_fn() {}

    // 이 함수는 my_module에서만 접근 가능합니다.
    fn public_fn() {}

    // 이 함수는 현재 크레이트에 정보를 제공합니다.
    pub(crate) fn public_fn() {}

    // 이 함수는 상위 모듈에 정보를 제공합니다.
    pub(super) fn public_fn() {}
}
