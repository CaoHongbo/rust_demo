// fn main() {
//     println!("Hello, world!");
// }


//extern crate chrono; //0.4
extern crate time; //0.1 
//use chrono::prelude::*;
// use std::thread;
use time::*;

fn fib(x: i64) -> i64 {
    match x < 2 {
        true => x,
        _ => fib(x - 2) + fib(x - 1),
    }
}
fn main() {
    let start = time::now(); //获取开始时间
    // let nums: Vec = vec![30_i64, 35, 40, 45];
    // for n in nums {
    let value = fib(43);
    // }
    let end = time::now(); //获取结束时间
    println!(
        "done!start : {:?},end :{:?},duration:{:?}",
        start,
        end,
        end - start
    );
    // thread::sleep_ms(500000);
}