//! # Eq
//!
//! `PartialEq`を実装していれば実装できるというわけではない
//!
//! `PartialEq`も実装する必要がある
//!
//! フィールドも`Eq`を実装する必要がある
//!
//! 必ず比較ができる場合に実装する

#[derive(Debug, Eq, PartialEq)]
struct Struct;

pub fn main() {}
