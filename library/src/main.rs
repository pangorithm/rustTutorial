struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Book 객체 해제: {}", self.title);
    }
}

fn main() {
    {
        let book = Book {
            title: String::from("러스트"),
        };
    } // book이 스코프를 벗어나며 book의 Drop 트레잇이 자동으로 호출된다.
}
