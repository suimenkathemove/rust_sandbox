//! # Clone
//!
//! 値のディープコピー
//!
//! 実装すると、要素に対して`clone`を呼び出すので、各フィールドも`Clone`を実装する必要がある

#[derive(Clone)]
struct Struct;

pub fn main() {
    let s1 = Struct {};
    let _s2 = s1.clone();
}
