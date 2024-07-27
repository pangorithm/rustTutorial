fn main() {}

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

enum SchoolKind {
    // 각 열거자에 구조체를 할당
    Elementary(ElementarySchool),
    Middle(MiddleSchool),
    High(HighSchool),
}
