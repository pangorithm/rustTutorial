trait Pointable {
    // 인터페이스 정의
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

struct Point {
    x: i32,
    y: i32,
}

impl Pointable for Point {
    // Point에 Pointable 인터페이스를 구현한다.
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

fn print_pointable(pointable: &dyn Pointable) {
    println!("x: {}, y: {}", pointable.x(), pointable.y());
}

// ColorPoint는 Point를 확장한 하위 클래스처럼 작동한다.
// 러스트는 상속을 제공하지 않기 때문에 내부적으로 point 인스턴스를 가지고 있다.
// 그리고 부모 클래스의 함수를 호출하는 것처럼 만들기 위해 point 인스턴스의 함수를 호출한다.
struct ColorPoint {
    color: String,
    point: Point,
}

impl ColorPoint {
    fn new(color: String, x: i32, y: i32) -> ColorPoint {
        ColorPoint {
            color: color,
            point: Point { x: x, y: y },
        }
    }

    fn color(&self) -> &String {
        &self.color
    }
}

impl Pointable for ColorPoint {
    fn x(&self) -> i32 {
        self.point.x
    }

    fn y(&self) -> i32 {
        self.point.y
    }
}

fn main() {
    let pt = ColorPoint::new(String::from("red"), 1, 2);
    print_pointable(&pt);
    // x: 1, y: 2
}
