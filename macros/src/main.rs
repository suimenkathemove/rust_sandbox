macro_rules! add {
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
}

fn main() {
    println!("{}", add!(1, 2));
}
