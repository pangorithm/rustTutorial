pub mod business;
pub mod database;
pub mod presentation;

#[test]
fn it_works() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}
