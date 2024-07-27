use std::io;

fn main() {
    // loop 반복문
    loop {
        // 다른 언어의 while (true)와 같다
        println!("숫자를 입력해주세요. 0을 입력하면 종료합니다.");
        let mut read = String::new();
        io::stdin().read_line(&mut read).unwrap();
        let val: i32 = read.trim().parse().unwrap();

        if val == 0 {
            break; // 종료
        }

        println!("입력={}", val);
    }
    println!("");

    // for 반복문
    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        // arr를 순회한다.
        print!("{}, ", a);
    }
    println!("");

    for a in 0..5 {
        // arr를 순회한다.
        print!("{}, ", a);
    }
    println!("");

    // while 반복문
    let mut counter = 0;
    while counter < 5 {
        print!("{}, ", counter);
        counter += 1;
    }
    println!("");
}
