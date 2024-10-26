use hyper::body::{self, Buf};
use hyper::{client, Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let url = "http://jsonplaceholder.typicode.com/users".parse().unwrap();

    let client = Client::new();
    let res = client.get(url).await.unwrap();
    let body = hyper::body::aggregate(res).await.unwrap();

    let users: Vec<User> = serde_json::from_reader(body.reader()).unwrap(); // 받은 JSON을 serde로 역직렬화

    println!("사용자: {:#?}", users);
}
