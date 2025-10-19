mod chrono;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    chrono::main();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", now);
}
