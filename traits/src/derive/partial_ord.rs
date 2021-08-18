//! # PartialOrd
//!
//! `PartialEq`も実装する必要がある
//!
//! `partial_cmp`メソッドが実装され、これは`Option<Ordering>`を返す

use std::f64::NAN;

#[derive(PartialEq, PartialOrd)]
struct Struct {
    item1: i32,
    item2: i32,
}

#[derive(PartialEq, PartialOrd)]
enum Enum {
    Item1,
    Item2,
}

pub fn main() {
    {
        assert_eq!((0.).partial_cmp(&NAN), None);
    }

    {
        let s1 = Struct { item1: 0, item2: 0 };
        let s2 = Struct { item1: 0, item2: 1 };
        // 構造体のフィールドの順番でフィールドを比較することでインスタンスを比較する
        assert!(s1 < s2);
    }

    {
        let e1 = Enum::Item1;
        let e2 = Enum::Item2;
        // 先に定義された列挙子がより小さいとする
        assert!(e1 < e2);
    }
}
