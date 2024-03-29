//! # 実行コマンドオプション
//!
//! ## 既定動作
//!
//! 並列に実行し、テスト実行中の出力をさせない
//!
//! ```bash
//! cargo test [options] [testname] [-- test-options]
//! ```
//!
//! `cargo test`で使用できるオプションを表示
//!
//! ```bash
//! cargo test --help
//! ```
//!
//! テストバイナリにかかる引数を表示
//!
//! ```bash
//! cargo test -- --help
//! ```
//!
//! テストスレッドを変更する（1にすることで逐次実行する）
//!
//! ```bash
//! cargo test -- --test-threads=1
//! ```
//!
//! ## 標準出力に出力するメッセージ
//!
//! デフォルトでは、テストが失敗しないと標準出力に出力されたメッセージを表示しない
//!
//! 成功時もメッセージを出力したい場合は`--nocapture`フラグを使用する
//!
//! ```bash
//! cargo test -- --nocapture
//! ```
//!
//! ## 特定のテストを実行する
//!
//! 関数名やモジュール名の一部を渡すことで、それらを含むテスト全てを実行する
//!
//! ```bash
//! cargo test [testname]
//! ```

#[test]
fn ok() {
    println!("成功しました");
}

#[test]
#[should_panic]
fn fail() {
    println!("失敗します");

    panic!();
}
