//! # Eq
//!
//! `PartialEq`を実装していれば実装できるというわけではない
//!
//! `PartialEq`も実装する必要がある
//!
//! フィールドも`Eq`を実装する必要がある

#[derive(Debug, Eq, PartialEq)]
struct Struct;

pub fn main() {}
