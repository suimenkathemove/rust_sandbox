#[cfg_attr(feature = "foo", derive(Debug))]
struct Foo {}

pub fn main() {
    let foo = Foo {};
    println!("{:?}", foo);
}
