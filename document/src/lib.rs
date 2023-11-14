//! # document
//!
//! ソースコード中にdocumentation commentを書くと、それを元にドキュメントを生成できる。
//! documentation commentにはmarkdownを用いることができる。
//! documentation commentを生成するには`cargo doc`を実行する。
//!
//! ## crateやmodule
//!
//! crateやmoduleのドキュメントは、ファイルの先頭に`//!`の後に書く。
//! crate rootがcrateのドキュメントのトップになるので、何のcrateかや、基本的な利用例を書くとよい。
//!
//! ## 型や関数
//!
//! 型や関数のドキュメントは、それらの直上に`///`の後に書く。
//! よくある項目は、Examples, Errors, Panicsなどである。
//!
//! ## document test
//!
//! documentation comment中の、```で囲まれたコードをテストすること。
//! `cargo test`でテストを実行する。

mod module {
    //! インラインのmoduleにドキュメントを書く場合はこのように書く。
}

/// 関数にドキュメントを書く場合はこのように書く。
/// ```
/// use document::function;
/// function();
/// ```
pub fn function() {}
