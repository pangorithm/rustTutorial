use std::env;

fn main() {
    for (index, argument) in env::args().enumerate() {
        println!("인자 #{}: {}", index, argument);
    }
}
/*
# cargo run test
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/system_call test`
인자 #0: target/debug/system_call
인자 #1: test
*/
