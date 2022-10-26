use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", now);
}
