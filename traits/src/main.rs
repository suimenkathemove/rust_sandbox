//! # traits
//!
//! インターフェースのようなもの
//!
//! ## 注意点
//!
//! トレイトか対象の型がクレート固有の場合のみ、そのトレイトをその型に実装できる
//!
//! 外部のトレイトを外部の型に実装できない
//!
//! 同じトレイトを同じ型に実装できてしまうので、どの実装を使うのかわからなくなるため

mod derive;
mod shape;
mod std_;
mod trait_object;

fn main() {
    derive::main();
    trait_object::main();
}
