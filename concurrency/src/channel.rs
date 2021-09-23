use std::{sync::mpsc, thread};

pub fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("{}", received);
}
