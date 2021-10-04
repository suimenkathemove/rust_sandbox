//! # スマートポインタ
//!
//! ## ポインタ
//!
//! - 参照
//! - スマートポインタ
//!
//! ## 参照
//!
//! データを参照する。オーバーヘッドがない
//!
//! データを借用する
//!
//! ## スマートポインタとは
//!
//! データの参照に加えて、追加のメタデータ、機能がある
//!
//! データを所有する
//!
//! `Deref`トレイトと`Drop`トレイトを実装した構造体
//!
//! ### 例
//!
//! - String
//! - Vec<T>

mod box_;
mod deref;
mod drop;
mod rc;

fn main() {
    box_::main();
    rc::main();
}
