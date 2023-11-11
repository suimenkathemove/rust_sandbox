//! modules
//!
//! moduleの定義方法には、modキーワードを用いる方法と、ファイル単位で分割する方法がある。
//! ファイル単位で分割する方法には、以下の2種類がある。
//! - module1
//!   - mod.rs
//!   - child.rs
//! - module2.rs
//! - module2
//!   - child.rs

// この箇所にmoduleが展開される
mod module1;
mod module2;
