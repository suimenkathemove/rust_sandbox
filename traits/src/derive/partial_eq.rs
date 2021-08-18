//! # PartialEq
//!
//! 構造体にderiveすると全フィールドが等しい場合のみ等価になる

#[derive(Debug, PartialEq)]
struct Struct {
    item: i32,
}

impl Struct {
    fn new() -> Self {
        Struct { item: 0 }
    }
}

#[derive(Debug, PartialEq)]
enum Enum {
    Item1,
    Item2,
}

pub fn main() {
    {
        let s1 = Struct::new();
        let s2 = Struct::new();
        assert_eq!(s1, s2);

        let s2 = Struct { item: 1 };
        assert_ne!(s1, s2);
    }

    {
        let e1 = Enum::Item1;
        let e2 = Enum::Item2;
        assert_ne!(e1, e2);
    }
}
