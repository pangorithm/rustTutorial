// 프레젠테이션 레이어를 나타내는 모듈
mod presentation {
    // 뷰에 관련된 기능을 담당하는 모듈
    mod view {
        // 렌더링에 관련된 함수
        fn render() {
            println!("mysystem::presentation::view::render");
        }
    }
}

// 비즈니스 로직을 담당하는 모듈
mod business {
    // 사용자 관련 비즈니스 로직을 담당하는 모듈
    mod user {
        // 사용자 생성에 관련된 함수
        fn create() {
            println!("mysystem::business::user::create");
        }
    }
}

// 데이터베이스 작업을 담당하는 모듈
mod database {
    // 사용자 데이터 액세스 객체(DAO)를 나타내는 모듈
    mod user_dao {
        // 사용자 생성에 관련된 데이터베이스 함수
        fn create() {
            println!("mysystem::database::user_dao::create");
        }
    }
}

#[test]
fn it_works() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}
