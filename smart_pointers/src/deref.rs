//! Deref
//!
//! 参照やスマートポインタなどのポインタに対して参照外し演算子を使うことで、値自体を使えるようになる
//!
//! 参照外し演算子を使うには、`Deref`トレイトを実装する必要がある
//!
//! `Deref`トレイトの`deref`メソッドが、参照外しが可能な`&`参照を返すことで参照外しができるようになる
//!
//! ## 参照外し型強制
//!
//! 関数やメソッドの引数に、`Deref`を実装する型の値への参照を渡した際に、`Deref`が変換できる型への参照に変換する
//!

#[test]
fn basic() {
    let x = 0;
    let y = &x;

    assert_eq!(&0, y);
    assert_eq!(0, *y);
}

#[test]
fn use_box() {
    let x = 0;
    let y = Box::new(x);

    assert_eq!(Box::new(0), y);
    assert_eq!(0, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn use_my_box() {
    let x = 0;
    let y = MyBox::new(x);

    assert_eq!(0, *y);
}

#[test]
fn implicit_deref_coercions() {
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    {
        let name = MyBox::new("name".to_string());
        // 内部でderefを呼び出すことで&Stringに変換している
        // さらにderefを呼び出し、&Stringを&strに変換している
        greet(&name);

        let name = "name".to_string();
        greet(&name);
    }

    {
        let name = MyBox::new("name".to_string());
        // 参照外し型強制を使用しない場合
        greet(&(*name)[..]);
    }

    {
        let mut name = MyBox::new("name".to_string());
        // 可変参照を不変参照に型強制することもできる（逆はできない）
        greet(&name);
    }
}
