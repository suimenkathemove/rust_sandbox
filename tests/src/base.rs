//! ## `assert_eq!`, `assert_ne!`

//! 引数の順番は基本、`expected`, `actual`

//! `Debug`トレイトと`PartialEq`トレイトを実装する必要がある

#[derive(Debug, PartialEq)]
struct Struct;

#[test]
fn test() {
    let s1 = Struct {};
    let s2 = Struct {};
    assert_eq!(s1, s2);
}
