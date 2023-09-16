//! # Rc
//!
//! 単独の値に対して、複数の所有権を可能にする
//!
//! reference countingの略
//!
//! 値への参照の数を追跡する
//!
//! 値への参照が0の場合、値は片付けられる
//!
//! 複数箇所で読むデータをヒープに確保したいが、コンパイル時にはどの部分が最後にデータを使用し終わるか決定できないときに`Rc`型を使用する
//!
//! `Rc`はシングルスレッドの文脈でのみ使用する
//!
//! `Rc::clone`は参照カウントをインクリメントするのみ
//!
//! ## TODO
//!
//! - https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("{}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }

    println!("{}", Rc::strong_count(&a));
}
