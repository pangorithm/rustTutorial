fn main() {}

struct Score {
    score: i32,
}

impl Score {
    fn get_grade(&self) -> String {
        if self.score > 90 {
            String::from("A")
        } else if self.score > 80 {
            String::from("B")
        } else {
            String::from("C")
        }
    }
}

// 테스트 함수를 정의하기 위한 어트리뷰트
#[test]
fn test_get_grade() {
    // Score 구조체 인스턴스 생성, 점수는 100
    let score = Score { score: 100 };

    // score.get_grade가 "A"를 반환하는지 검사
    assert_eq!(score.get_grade(), "A");

    // Score 구조체 인스턴스 생성, 점수는 80
    let score = Score { score: 80 };

    // score.get_grade가 "A"를 반환하는지 검사
    assert_eq!(score.get_grade(), "B");
}
