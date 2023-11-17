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
//!
//! Rustが標準で提供するチャネルは、送信は複数スレッドから可能だが、受信は1スレッドからしかできない。
//! 複数スレッドから受信したい場合は、crossbeam_channelなどを使うとよい。
//!
//! ## sync_channel
//!
//! sync_channelはチャネル内部に保持可能なバッファのサイズが有限で、channelには制限はない。
//! sync_channelでは、バッファを使い果たしてしまった場合に送信すると、プログラムはブロックする。つまり、sendを呼び出したスレッドのプログラムは、バッファに空きができるまで停止する。
//! 一方、channelでは、バッファに空きがない場合には、バッファのサイズを動的に大きくして送信する。
//! channelの場合はチャネル自体が大量にメモリを消費してしまう可能性があるので、ほとんどの場合はsync_channelで問題ない。

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn single() {
        // txが送信端、rxが受信端
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = "hi".to_string();
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();

        println!("{}", received);
    }

    #[test]
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

    #[test]
    fn clone_tx() {
        let (tx, rx) = mpsc::channel();

        // 複数スレッドから送信したい場合はtxをcloneする
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
}
