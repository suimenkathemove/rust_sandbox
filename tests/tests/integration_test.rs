//! # 結合テスト
//!
//! 公開APIしかテストできない
//!
//! `/tests`に置き、テストファイルを個別のクレートとしてコンパイルする
//!
//! 特定の結合テストを実行するコマンド
//!
//! ```bash
//! cargo test --test [filename]
//! ```
//!
//! `/tests`ディレクトリのサブディレクトリ内のファイルは個別クレートとしてコンパイルされない

mod common;

use tests;

#[test]
fn one_plus_two_is_three() {
    common::setup();

    assert_eq!(3, tests::add(1, 2));
}
