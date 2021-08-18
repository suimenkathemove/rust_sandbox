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
    item1,
    item2,
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
        let e1 = Enum::item1;
        let e2 = Enum::item2;
        assert_ne!(e1, e2);
    }
}
