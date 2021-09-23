use std::{sync::mpsc, thread, time::Duration};

fn single() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = "hi".to_string();
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();

    println!("{}", received);
}

fn multiple() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let nums = vec![1, 2, 3];
        for num in nums {
            tx.send(num).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}

fn clone_tx() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let nums = vec![1, 2, 3];
        for num in nums {
            tx1.send(num).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let nums = vec![4, 5, 6];
        for num in nums {
            tx.send(num).unwrap();

            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}

pub fn main() {
    single();

    multiple();

    clone_tx();
}
