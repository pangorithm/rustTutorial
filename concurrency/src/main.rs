use futures::executor::block_on;

// async 키워드를 사용해 비동기 함수를 정의합니다.
async fn hello_world() {
    println!("future 안에서 실행");
}

fn main() {
    let future = hello_world(); // 함수가 바로 호출되지 않습니다.
    println!("main 안에서 실행");

    // future를 실행합니다. hello_world가 종료될 때까지 main thread는 멈춥니다.
    block_on(future);
    println!("future 종료 이후 실행");
}
