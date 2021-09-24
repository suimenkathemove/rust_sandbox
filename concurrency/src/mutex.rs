use std::sync::Mutex;

pub fn main() {
    let m = Mutex::new(0);

    {
        // ロックを得るまで現在のスレッドをブロックする
        // ロックを保持している他のスレッドがパニックしたら、lockの呼び出しはエラーになる
        let mut num = m.lock().unwrap();
        *num = 1;
        // スコープを外れたときにロックを解除するDrop実装もしている
    }

    println!("{:?}", m);
}
