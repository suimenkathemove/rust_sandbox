//! # modules
//!
//! moduleの定義方法には、modキーワードを用いる方法と、ファイル単位で分割する方法がある。
//! ファイル単位で分割する方法には、以下の2種類がある。
//! - module1
//!   - mod.rs
//!   - child.rs
//! - module2.rs
//! - module2
//!   - child.rs
//!
//! ## 可視性 visibility
//!
//! 子モジュールの要素はpublicなものしか見ることができない。
//! モジュールは壁のようなもの。壁内の要素は見ることができる。壁の外側には出ることができる。子の壁の内側はpublicな要素しか見ることができない。
//! フィールドやメソッドについて、pubを付けないとアクセスできない要素のそれらには、pubを付けないとアクセスできない。
//!
//! pubの指定方法
//! - pub(self) pubを省略したものと同じ。
//! - pub(super)
//! - pub(crate) crate内でのみ利用できるようにする。library crateではよく利用される。
//! - pub(in path) 公開する祖先のモジュールを指定する。
//!
//! crateから始まるパスはcrate rootを始祖とした絶対パスであり、それ以外は相対パスになる。
//!
//! ## use
//!
//! importやre-exportをするために用いられる。pub useを用いることでre-exportできる。

// この箇所にmoduleが展開される
mod module1;
mod module2;
