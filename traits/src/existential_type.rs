//! # existential type
//!
//! 存在型(existential type)とは、型をより抽象的に表現するための型である。
//! 引数や返り値の型に、`impl Trait`と書く。
//!
//! ## use case
//!
//! - クロージャの型。クロージャの型はクロージャごとに異なるため。
//! - 複雑な型を表現する場合。
//!
//! 存在型は静的ディスパッチであるため、複数の型の値を返すような関数には利用できない。その場合は動的ディスパッチを利用する。

#[test]
fn test_return_closure() {
    // `Fn`は関数型を表すtraitであり、関数やクロージャはこれを自動的に実装する。
    fn return_closure() -> impl Fn(i32) -> i32 {
        |x| x + x
    }
    assert_eq!(2, return_closure()(1));
}

#[test]
fn closure_type_is_unique() {
    use std::any::{Any, TypeId};

    let f = |x: i32| x + x;
    let g = |x: i32| x + x;

    // 型情報を取得する
    fn get_type_id<T: Any>(_: &T) -> TypeId {
        TypeId::of::<T>()
    }

    // 型情報を比較する
    assert_ne!(get_type_id(&f), get_type_id(&g));
}
