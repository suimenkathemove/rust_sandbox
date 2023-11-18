//! # mutual exclusion
//!
//! 排他制御(mutual exclusion)とは、並行に動作するプロセスが共有リソースへ同時にアクセスすることを制限すること。
//!
//! 競合状態(race condition)とは、複数のスレッドが並行に動作して引き起こされる不正な状態のこと。

#[cfg(test)]
mod mutex {
    //! ## mutex
    //!
    //! 読み込みも書き込みも1つのスレッドしか実行できない。

    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    #[test]
    fn sample() {
        let x = Arc::new(Mutex::new(100_000));
        let x2 = Arc::clone(&x);

        let a = thread::spawn(move || {
            // ロックを獲得するまで現在のスレッドをブロックする
            // ロックを獲得している他のスレッドがパニックしたら、lockの呼び出しはエラーになる
            // guardが生存している間、guardを参照して保護対象のデータにアクセスできる
            let mut guard = x.lock().unwrap();
            *guard -= 20_000;
            // guardの生存期間が終了すると、ロックは自動的に解放される
        });

        let b = thread::spawn(move || {
            let mut guard = x2.lock().unwrap();
            *guard -= 30_000;
        });

        a.join().unwrap();
        b.join().unwrap();
    }
}

#[cfg(test)]
mod rw_lock {
    //! ## RW lock
    //!
    //! reader writer lock; RW lock
    //! 読み込みのみを行うreaderと、読み書きを行うwriterがある。
    //! ロック獲得中のwriterは最大1つである。
    //! readerとwriterのどちらか一方がロックを獲得中の場合は、もう一方はロックを獲得できない。
}
