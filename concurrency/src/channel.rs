//! # message passing
//!
//! スレッド間やアクター間で、データを含むメッセージを転送する
//!
//! 川のようなもので、上流の転送機と下流の受信機に分かれる
//!
//! 転送機と受信機のどちらかがドロップされると、チャンネルが閉じられる
//!
//! `mpsc`⋯multiple producer, single consumer
//!
//! メッセージをチャンネルを通して送信できるように、スレッドがチャンネルの送信側を所有する必要がある
//!
//! `recv`⋯呼び出し側のスレッドをブロックし、値を受け取るまで待つ。送信側が閉じたらエラーを返す
//!
//! `try_recv`⋯ブロックせず、即座に`Result<T, E>`を返す。メッセージを待つ間に何かしたい場合に使う

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
