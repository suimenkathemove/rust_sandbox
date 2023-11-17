#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread};

    #[test]
    fn main_send_thread_receive() {
        let (tx, rx) = mpsc::channel();

        tx.send(1).unwrap();
        println!("main: send value");

        thread::spawn(move || {
            println!("thread: waiting...");
            let value = rx.recv().unwrap();
            println!("thread: receive {}", value);
        });

        println!("main: end");
    }

    #[test]
    fn thread_receive_main_send() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            println!("thread: waiting...");
            let value = rx.recv().unwrap();
            println!("thread: receive {}", value);
        });

        tx.send(1).unwrap();
        println!("main: send value");
    }

    #[test]
    fn thread_send_main_receive() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(1).unwrap();
            println!("thread: send value");
        });

        println!("main: waiting...");
        let value = rx.recv().unwrap();
        println!("main: receive {}", value);
    }

    // これは終わらない
    #[test]
    fn main_receive_thread_send() {
        let (tx, rx) = mpsc::channel();

        println!("main: waiting...");
        let value = rx.recv().unwrap();
        println!("main: receive {}", value);

        thread::spawn(move || {
            tx.send(1).unwrap();
            println!("thread: send value");
        });

        println!("main: end");
    }
}
