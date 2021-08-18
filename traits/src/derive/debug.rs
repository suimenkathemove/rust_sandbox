#[derive(Debug)]
struct Struct {
    item: i32,
}

impl Struct {
    fn new() -> Self {
        Struct { item: 0 }
    }
}

#[derive(Debug)]
enum Enum {
    Item,
}

pub fn main() {
    let s = Struct::new();
    assert_eq!(format!("{:?}", s), "Struct { item: 0 }");

    let e = Enum::Item;
    assert_eq!(format!("{:?}", e), "Item");
}
