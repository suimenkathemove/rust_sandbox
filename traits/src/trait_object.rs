//! # トレイトオブジェクト
//!
//! ## トレイトオブジェクトとは
//!
//! - トレイトを実装する型のインスタンス。何らかのポインタを指定し、そして、トレイトを指定することでトレイトオブジェクトを作成する
//! - ジェネリックや具体的な型があるところで使用可能
//! - データと振る舞いを混同するが、データを追加することはできず、振る舞いに関して抽象化する
//! - ジェネリックな型引数は、一度に1つの具体型にしか置き換えられないのに対して、トレイトオブジェクトは、実行時に複数の具体型で埋めることができる
//!
//! ### スタティックディスパッチ
//!
//! ジェネリクスに対してトレイト境界を使用した場合
//!
//! ### ダイナミックディスパッチ
//!
//! トレイトオブジェクトを使用した場合
//!
//! ## オブジェクト安全性が必要
//!
//! コンパイラはトレイトを実装している具体的な型を知らないので、トレイトはオブジェクト安全である必要がある
//!
//! トレイトは、トレイト内で定義されているメソッド全てに以下の特性があれば、オブジェクト安全になる
//!
//! - 戻り値の型が`Self`でない
//! - ジェネリックな型引数がない

trait Draw {
    fn draw(&self);
}

struct Screen {
    // Box<Draw>はトレイトオブジェクト
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // ジェネリックな型引数は、一度に1つの具体型にしか置き換えられないのに対して、
    // トレイトオブジェクトは、実行時にトレイトオブジェクトに対して複数の具体型で埋めることができる
    fn run(&self) {
        for c in self.components.iter() {
            c.draw();
        }
    }
}

struct Button {
    _width: u32,
    _height: u32,
    _label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    _width: u32,
    _height: u32,
    _options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}

pub fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                _width: 1,
                _height: 1,
                _label: "OK".to_string(),
            }),
            Box::new(SelectBox {
                _width: 1,
                _height: 1,
                _options: vec!["Yes".to_string(), "No".to_string()],
            }),
        ],
    };
    screen.run();
}
