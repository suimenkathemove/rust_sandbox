//! # 構造体
//!
//! 一部のフィールドを可変にすることはできない

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// メソッドの第一引数は`self`（インスタンス）
    /// &を付けず`self`のみにすることは稀である
    /// メソッドが`self`を別のものに変形し、変形後に呼び出し元が元のインスタンスを使用できないようにしたい場合に使用される
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// `self`を引数に取らない関数はassociated functionsといい、構造体に関連付けられる
    /// 対象のインスタンスが存在しないので、メソッドではなく関数である
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn func() {
    {
        let rectangle = Rectangle {
            width: 2,
            height: 3,
        };
        let area = rectangle.area();
        println!("{}", area);
    }

    {
        let square = Rectangle::square(1);
    }
}
