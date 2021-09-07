//! Drop
//!
//! 値がスコープを抜ける時に実行するコードをオーバーライドでき、例えばファイルやネットワーク接続などのリソースを解放するのに活用できる
//!
//! どんな型に対しても`Drop`トレイトを実装できる
//!
//! `Drop`トレイトの機能は、ほぼスマートポインタを実装する際に使われる
//!
//! `Box<T>`は`Drop`をカスタマイズして、ボックスが指しているヒープの領域を解放している
//!
//! 変数は、生成するのとは逆の順序でドロップされる

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

#[test]
fn use_custom_smart_pointer() {
    println!("start");

    {
        // 変数は、生成するのとは逆の順序でドロップされる
        let _foo = CustomSmartPointer {
            data: "foo".to_string(),
        };
        let _bar = CustomSmartPointer {
            data: "bar".to_string(),
        };
    }

    println!("end");
}

#[test]
fn use_drop() {
    let foo = CustomSmartPointer {
        data: "foo".to_string(),
    };
    // Dropトレイトのdropメソッドは手動で実行できないので、代わりにstd::mem::dropを呼ぶ
    drop(foo);

    println!("end");
}
