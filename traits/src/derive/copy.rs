//! # Copy
//!
//! `Clone`も実装する必要がある
//!
//! 要素も`Copy`を実装する必要がある
//!
//! 使うことは稀
//!
//! `Copy`で可能なことは`Clone`でも可能だが、コードが遅くなる可能性がある

#[derive(Clone, Copy)]
struct Item;

#[derive(Clone, Copy)]
struct Struct {
    _item: Item,
}

pub fn main() {}
