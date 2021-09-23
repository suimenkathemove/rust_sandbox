use std::thread;

pub fn main() {
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("{:?}", v);
    });
}
