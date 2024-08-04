// 렌더링에 관련된 함수
pub fn render() {
    println!("mysystem::presentation::view::render");
    // business::user::create(); // 빌드 오류 발생
    super::super::business::user::create(); // 각 모듈은 현재 위치를 기준으로 모듈을 찾으므로 ../와 같은 역할을 하는 super 키워드 사용
    crate::business::user::create(); // crate라는 키워드를 사용해 크레이트를 기준으로 모듈을 탐색
}
