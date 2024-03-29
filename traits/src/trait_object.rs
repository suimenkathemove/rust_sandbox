//! # トレイトオブジェクト
//!
//! トレイトオブジェクトとは、特定のトレイトを満たす任意の型のインスタンスを参照するための型。
//! 共通のトレイトを実装している異なる型を同様に扱うことができるようになる。
//!
//! 何らかのポインタ(参照やヒープ割り当て)を指定し、`dyn`キーワードとトレイトを指定することでトレイトオブジェクトを作成できる。
//!
//! ジェネリックな型引数は一度に1つの具体型にしか置き換えられないのに対して、トレイトオブジェクトは実行時に複数の具体型で埋めることができる。
//!
//! トレイトオブジェクトはファットポインタであり、1つ目のポインタは実装先の型を参照し、2つ目のポインタは関数テーブルへのポインタを参照する。
//! 関数テーブルとは、トレイトの各メソッドの実装を指すポインタの集合のこと。
//!
//! ## 静的ディスパッチ
//!
//! コンパイル時に、トレイトのメソッドの呼び出し先が決定されること。
//!
//! ジェネリクスに対してトレイト制約を使用した場合は、静的ディスパッチになる。
//!
//! ## 動的ディスパッチ
//!
//! 実行時に、トレイトのメソッドの呼び出し先が決定されること。
//!
//! トレイトオブジェクトを使用した場合は、動的ディスパッチになる。
//!
//! ## トレイトオブジェクトにはオブジェクト安全性が必要
//!
//! トレイトオブジェクトはオブジェクト安全なトレイトしか作成できない。
//! コンパイラはトレイトを実装している具体的な型を知らないので、トレイトはオブジェクト安全である必要がある。
//!
//! トレイトは、トレイト内で定義されているメソッド全てに以下の特性があれば、オブジェクト安全になる。
//! - 戻り値の型が`Self`でない
//! - ジェネリックな型引数がない

mod error {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    struct ErrorA;

    impl Display for ErrorA {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ErrorA")
        }
    }

    impl Error for ErrorA {}

    #[derive(Debug)]
    struct ErrorB;

    impl Display for ErrorB {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ErrorB")
        }
    }

    impl Error for ErrorB {}

    fn error_a() -> Result<(), ErrorA> {
        Err(ErrorA)
    }

    fn error_b() -> Result<(), ErrorB> {
        Err(ErrorB)
    }

    fn error_a_b() -> Result<(), Box<dyn Error>> {
        // ?でBox型に変換できている
        error_a()?;
        error_b()?;
        Ok(())
    }
}
