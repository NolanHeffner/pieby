// use std::io;

/* fn specify_func_and_param<F>(f: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let num = 3;
    let func = |x| x * x * x - 2;
    println!("{}", specify_func_and_param(func, num));
} */

fn main() {
    let num : u8 = 0b10001010;
    let numinv = !num;
    fn shift(&binNum: &u8) -> u8 {
        binNum << -1
    }
    println!("{:#b}, {:#b}, {:#b}", num, numinv, shift(&num));
}