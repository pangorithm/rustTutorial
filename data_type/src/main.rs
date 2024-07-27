fn main() {}

// 열거형 정의
enum SchoolKind {
    Elementary,
    Middle,
    High,
}

// 열거형 확장
struct ElementarySchool {
    room: String,
}
struct MiddleSchool {
    teacher: String,
}
struct HighSchool {
    id: i32,
}
