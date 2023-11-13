//! # document
//!
//! ソースコード中にdocumentation commentを書くと、それを元にドキュメントを生成できる。
//! documentation commentにはmarkdownを用いることができる。
//! documentation commentを生成するには`cargo doc`を実行する。
//!
//! ## crateやmodule
//!
//! crateやmoduleのドキュメントは、`//!`の後に書き、ファイルの先頭に書く。
//! crate rootがcrateのドキュメントのトップになるので、何のcrateかや、基本的な利用例を書くとよい。

mod module {
    //! インラインのmoduleにドキュメントを書く場合はこのように書く。
}

fn main() {
    println!("Hello, world!");
}
