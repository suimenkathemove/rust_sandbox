pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    pub fn x_abs(&self) -> i32 {
        self.x.abs()
    }
}

pub fn main() {
    let p = Point { x: -1, y: 1 };
    // i32型のみ呼び出すことができる
    assert_eq!(p.x_abs(), 1);
}
