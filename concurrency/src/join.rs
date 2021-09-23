use std::{thread, time::Duration};

pub fn main() {
    let join_handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("{}", i);

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 11..=15 {
        println!("{}", i);

        thread::sleep(Duration::from_millis(1));
    }

    // join_handleのスレッドが終了するまでメインスレッドをブロックする
    join_handle.join().unwrap();
}
