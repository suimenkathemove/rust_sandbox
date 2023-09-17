//! # スマートポインタ
//!
//! ## ポインタ
//!
//! メモリのアドレスを格納している変数
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
//! `Deref` トレイトにより、参照のように振る舞うことができる
//!
//! ### 例
//!
//! - String
//! - Vec<T>
//!
//! ### Box, Rc, RefCellの違い
//!
//! - `Rc`は同じデータに複数の所有者を持たせる
//! - `Box`は不変借用も可変借用もコンパイル時に精査。`Rc`は不変借用のみがコンパイル時に精査。`RefCell`は不変借用も可変借用も実行時に精査

mod box_;
mod deref;
mod drop;
mod rc_arc;