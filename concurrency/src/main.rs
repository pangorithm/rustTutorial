use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // 새로운 스레드를 생성하고 그 핸들을 받습니다.
        let file = File::open("file.txt").unwrap(); // "file.txt" 파일을 엽니다.
        let reader = BufReader::new(file); // 버퍼링을 사용해 파일을 읽습니다.
        for line in reader.lines() {
            // 파일의 각 줄을 순회합니다.
            let txt = line.unwrap(); // 줄을 텍스트로 읽습니다.
            println!("{}", txt);
        }
    });

    handle.join().unwrap(); // 스레드가 완료될 때까지 대기 (종료 대기)
}

// // # RUST_BACKTRACE=full cargo run
// Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/concurrency`
// thread '<unnamed>' panicked at src/main.rs:8:43:
// called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
// stack backtrace:
//    0:     0x555b0e31a785 - std::backtrace_rs::backtrace::libunwind::trace::h1a07e5dba0da0cd2
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
//    1:     0x555b0e31a785 - std::backtrace_rs::backtrace::trace_unsynchronized::h61b9b8394328c0bc
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
//    2:     0x555b0e31a785 - std::sys_common::backtrace::_print_fmt::h1c5e18b460934cff
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:68:5
//    3:     0x555b0e31a785 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e1a1972118942ad
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:44:22
//    4:     0x555b0e337deb - core::fmt::rt::Argument::fmt::h07af2b4071d536cd
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/rt.rs:165:63
//    5:     0x555b0e337deb - core::fmt::write::hc090a2ffd6b28c4a
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/mod.rs:1157:21
//    6:     0x555b0e3189bf - std::io::Write::write_fmt::h8898bac6ff039a23
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/io/mod.rs:1832:15
//    7:     0x555b0e31a55e - std::sys_common::backtrace::_print::h4e80c5803d4ee35b
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:47:5
//    8:     0x555b0e31a55e - std::sys_common::backtrace::print::ha96650907276675e
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:34:9
//    9:     0x555b0e31b819 - std::panicking::default_hook::{{closure}}::h215c2a0a8346e0e0
//   10:     0x555b0e31b55d - std::panicking::default_hook::h207342be97478370
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:298:9
//   11:     0x555b0e31bcb3 - std::panicking::rust_panic_with_hook::hac8bdceee1e4fe2c
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:795:13
//   12:     0x555b0e31bb94 - std::panicking::begin_panic_handler::{{closure}}::h00d785e82757ce3c
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:664:13
//   13:     0x555b0e31ac49 - std::sys_common::backtrace::__rust_end_short_backtrace::h1628d957bcd06996
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:171:18
//   14:     0x555b0e31b8c7 - rust_begin_unwind
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
//   15:     0x555b0e2f5003 - core::panicking::panic_fmt::hdc63834ffaaefae5
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
//   16:     0x555b0e2f5416 - core::result::unwrap_failed::h82b551e0ff2b2176
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1654:5
//   17:     0x555b0e2fdc1f - core::result::Result<T,E>::unwrap::h1277c665e9fe8e2c
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1077:23
//   18:     0x555b0e2fdc1f - concurrency::main::{{closure}}::h2ae32d4e23324a57
//                                at /root/git/rustTutorial/concurrency/src/main.rs:8:20
//   19:     0x555b0e2fa2b6 - std::sys_common::backtrace::__rust_begin_short_backtrace::h4c6d2a2844fd2526
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:155:18
//   20:     0x555b0e2fbe56 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::h8d0e69aa78341a30
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/thread/mod.rs:542:17
//   21:     0x555b0e300016 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hb3ebf4b385a10cc4
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panic/unwind_safe.rs:272:9
//   22:     0x555b0e2fe06c - std::panicking::try::do_call::h5aa18386106520e7
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:559:40
//   23:     0x555b0e2fe1cb - __rust_try
//   24:     0x555b0e2fe006 - std::panicking::try::hfecb6aa5f697c65f
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:523:19
//   25:     0x555b0e2fbb27 - std::panic::catch_unwind::h098882a4939bb4ef
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panic.rs:149:14
//   26:     0x555b0e2fbb27 - std::thread::Builder::spawn_unchecked_::{{closure}}::hbb69b282aa185f8d
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/thread/mod.rs:541:30
//   27:     0x555b0e2fe22e - core::ops::function::FnOnce::call_once{{vtable.shim}}::h79f3ae9130387026
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:250:5
//   28:     0x555b0e31db6b - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h09e5a4c541afa800
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/alloc/src/boxed.rs:2022:9
//   29:     0x555b0e31db6b - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c8b03c22f4e7026
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/alloc/src/boxed.rs:2022:9
//   30:     0x555b0e31db6b - std::sys::pal::unix::thread::Thread::new::thread_start::h522bc89a54da820a
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys/pal/unix/thread.rs:108:17
//   31:     0x7fda8aca3c02 - start_thread
//   32:     0x7fda8ad28c40 - clone3
//   33:                0x0 - <unknown>
// thread 'main' panicked at src/main.rs:17:19:
// called `Result::unwrap()` on an `Err` value: Any { .. }
// stack backtrace:
//    0:     0x555b0e31a785 - std::backtrace_rs::backtrace::libunwind::trace::h1a07e5dba0da0cd2
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/libunwind.rs:105:5
//    1:     0x555b0e31a785 - std::backtrace_rs::backtrace::trace_unsynchronized::h61b9b8394328c0bc
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
//    2:     0x555b0e31a785 - std::sys_common::backtrace::_print_fmt::h1c5e18b460934cff
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:68:5
//    3:     0x555b0e31a785 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1e1a1972118942ad
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:44:22
//    4:     0x555b0e337deb - core::fmt::rt::Argument::fmt::h07af2b4071d536cd
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/rt.rs:165:63
//    5:     0x555b0e337deb - core::fmt::write::hc090a2ffd6b28c4a
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/fmt/mod.rs:1157:21
//    6:     0x555b0e3189bf - std::io::Write::write_fmt::h8898bac6ff039a23
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/io/mod.rs:1832:15
//    7:     0x555b0e31a55e - std::sys_common::backtrace::_print::h4e80c5803d4ee35b
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:47:5
//    8:     0x555b0e31a55e - std::sys_common::backtrace::print::ha96650907276675e
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:34:9
//    9:     0x555b0e31b819 - std::panicking::default_hook::{{closure}}::h215c2a0a8346e0e0
//   10:     0x555b0e31b55d - std::panicking::default_hook::h207342be97478370
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:298:9
//   11:     0x555b0e31bcb3 - std::panicking::rust_panic_with_hook::hac8bdceee1e4fe2c
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:795:13
//   12:     0x555b0e31bb94 - std::panicking::begin_panic_handler::{{closure}}::h00d785e82757ce3c
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:664:13
//   13:     0x555b0e31ac49 - std::sys_common::backtrace::__rust_end_short_backtrace::h1628d957bcd06996
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:171:18
//   14:     0x555b0e31b8c7 - rust_begin_unwind
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
//   15:     0x555b0e2f5003 - core::panicking::panic_fmt::hdc63834ffaaefae5
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
//   16:     0x555b0e2f5416 - core::result::unwrap_failed::h82b551e0ff2b2176
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1654:5
//   17:     0x555b0e2f577a - core::result::Result<T,E>::unwrap::h11e29412cf5b4b52
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1077:23
//   18:     0x555b0e2f577a - concurrency::main::hb385786af0809a00
//                                at /root/git/rustTutorial/concurrency/src/main.rs:17:5
//   19:     0x555b0e2fe2fb - core::ops::function::FnOnce::call_once::hf7bfee49785e0698
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:250:5
//   20:     0x555b0e2fa2ce - std::sys_common::backtrace::__rust_begin_short_backtrace::ha3342739eecd3fd9
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/sys_common/backtrace.rs:155:18
//   21:     0x555b0e2fa521 - std::rt::lang_start::{{closure}}::h4d6ada9cf051ce12
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:159:18
//   22:     0x555b0e315c80 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h6abeee5a7794ceb5
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/ops/function.rs:284:13
//   23:     0x555b0e315c80 - std::panicking::try::do_call::hd6e966bb06877057
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:559:40
//   24:     0x555b0e315c80 - std::panicking::try::hc9b3807f5768cb19
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:523:19
//   25:     0x555b0e315c80 - std::panic::catch_unwind::h94a757c154076c6e
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panic.rs:149:14
//   26:     0x555b0e315c80 - std::rt::lang_start_internal::{{closure}}::hc5223fb36050c743
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:141:48
//   27:     0x555b0e315c80 - std::panicking::try::do_call::hddf7b4e1ebeb3f69
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:559:40
//   28:     0x555b0e315c80 - std::panicking::try::h1842860a1f941a31
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:523:19
//   29:     0x555b0e315c80 - std::panic::catch_unwind::h009016ccf811d4c3
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panic.rs:149:14
//   30:     0x555b0e315c80 - std::rt::lang_start_internal::h3ed4fe7b2f419135
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:141:20
//   31:     0x555b0e2fa4fa - std::rt::lang_start::haf7d5fe232ea9113
//                                at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/rt.rs:158:17
//   32:     0x555b0e2f57ce - main
//   33:     0x7fda8ac43590 - __libc_start_call_main
//   34:     0x7fda8ac43640 - __libc_start_main_alias_1
//   35:     0x555b0e2f55f5 - _start
//   36:                0x0 - <unknown>
