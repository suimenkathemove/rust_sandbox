#![allow(dead_code)]

mod point;

fn main() {
    let p = point::Point { x: -1, y: 1 };
    // i32型のみ呼び出すことができる
    assert_eq!(p.x_abs(), 1);
}
