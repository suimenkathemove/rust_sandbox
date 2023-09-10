#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn spawn() {
        // spawnでスレッドを生成する
        thread::spawn(|| {
            for i in 1..=10 {
                println!("{}", i);

                // OSがスレッドのスケジュールを行う方法に依存するため、スレッドが切り替わる保証はない
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 11..=15 {
            println!("{}", i);

            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn spawn_join() {
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

        // join()で、join_handleのスレッドが終了するまでメインスレッドをブロックする
        join_handle.join().unwrap();
    }
}
