// use std::io;
mod uci;
mod search;


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
/*     let num : u8 = 0b10001010;
    let numinv = !num;
    fn shift(&binNum: &u8) -> u8 {
        binNum << -1
    }
    println!("{:#b}, {:#b}, {:#b}", num, numinv, shift(&num)); */
    /* let arr : [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("{}", arr[1][2]); */
    let engine: uci::UCIEngine = uci::UCIEngine::new("pieby", "Nolan Heffner and Matthew Burger", "v0.1");
    engine.uci_loop();
}