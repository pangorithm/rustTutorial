use sqlite;
use sqlite::State;

fn main() {
    let connection = sqlite::open(":memory:").unwrap(); // 메모리에 sqlite db를 생성

    let query = "
      CREATE TABLE users (name TEXT, age INTEGER);
      INSERT INTO users VALUES ('루나', 3);
      INSERT INTO users VALUES ('러스트', 13);
    ";
    connection.execute(query).unwrap(); // 테이블 생성 쿼리를 실행

    let query = "SELECT * FROM users WHERE age > ?"; // ?는 1에 바인딩 됨
    let mut statement = connection.prepare(query).unwrap(); // 쿼리를 실행
    statement.bind((1, 5)).unwrap(); // age > 5

    while let Ok(State::Row) = statement.next() {
        // 테이블의 데이터를 조회
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());
    }
}
