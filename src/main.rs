// use timer;
// use chrono;
// use std::thread;
// use std::sync::{Arc, Mutex};

// fn main() {
//     let timer = timer::Timer::new();
//     // Number of times the callback has been called.
//     let count = Arc::new(Mutex::new(0));

//     // Start repeating. Each callback increases `count`.
//     let guard = {
//       let count = count.clone();
//       timer.schedule_repeating(chrono::Duration::nanoseconds(500), move || {
//         *count.lock().unwrap() += 1;
//       })
//     };

//     // Sleep one second. The callback should be called ~200 times.
//     thread::sleep(std::time::Duration::new(1, 0));
//     let count_result = *count.lock().unwrap();
//     print!("{count_result}");
//     assert!(190 <= count_result && count_result <= 210,
//       "The timer was called {} times", count_result);

//     // Now drop the guard. This should stop the timer.
//     drop(guard);
//     thread::sleep(std::time::Duration::new(0, 100));

//     // Let's check that the count stops increasing.
//     let count_start = *count.lock().unwrap();
//     thread::sleep(std::time::Duration::new(1, 0));
//     let count_stop =  *count.lock().unwrap();
//     assert_eq!(count_start, count_stop);
// }


// An integer division that doesn't `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is represented as the `None` variant
        None
    } else {
        // Result is wrapped in a `Some` variant
        Some(dividend / divisor)
    }
}

// This function handles a division that may not succeed
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values can be pattern matched, just like other enums
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable needs to be type annotated
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unwrapping a `Some` variant will extract the value wrapped.
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unwrapping a `None` variant will `panic!`
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}
