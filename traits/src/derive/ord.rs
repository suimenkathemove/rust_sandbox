//! # Ord
//!
//! 常に順序付けが可能な`cmp`メソッドを実装し、これは`Ordering`を返す
//!
//! `cmp`は、`Partial`の`partial_cmp`と同じように振る舞う
//!
//! `PartialOrd`と`Eq`も実装する必要がある
//!
//! 必ず比較ができる場合に実装する

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Struct;

pub fn main() {
    let s1 = Struct {};
    let s2 = Struct {};

    let _ordering = s1.cmp(&s2);
}
