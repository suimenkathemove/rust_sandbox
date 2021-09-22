use std::{thread, time::Duration};

pub fn main() {
    // 新規スレッドはメインスレッドが終了したら停止する
    thread::spawn(|| {
        for i in 1..=10 {
            println!("{}", i);

            // 少しの間スレッドの実行を止め、違うスレッドを走らせることができる
            // スレッドが切り替わる保証はない（OSがスレッドのスケジュールを行う方法に依存するため）
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 11..=15 {
        println!("{}", i);

        thread::sleep(Duration::from_millis(1));
    }
}
