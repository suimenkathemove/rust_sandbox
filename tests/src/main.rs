//! # テスト
//! ## テスト関数本体の構成
//!
//! 1. セットアップ
//! 2. テスト対象の実行
//! 3. アサーション
//!
//! ## 全てのテストを実行
//!
//! ```bash
//! cargo test
//! ```
//!
//! ## ベンチマークテスト
//!
//! [ベンチマークテスト](https://doc.rust-lang.org/unstable-book/library-features/test.html)

mod base;
mod ignore;
mod message;
mod options;
mod should_panic;
mod unit_tests;

fn main() {}
