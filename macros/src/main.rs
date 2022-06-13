use std::fmt::Display;

macro_rules! add {
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
}

macro_rules! call {
    ($func: ident, $e: expr) => {
        $func($e)
    };
}

fn my_print<T: Display>(arg: T) {
    println!("{}", arg);
}

fn main() {
    println!("{}", add!(1, 2));
    call!(my_print, 3);
}
