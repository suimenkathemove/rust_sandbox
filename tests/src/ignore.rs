//! # テストを除外する
//!
//! `ignore`属性を指定する
//!
//! ignoreしているテストのみを実行したい場合
//!
//! ```bash
//! cargo test -- --ignored
//! ```

#[test]
#[ignore]
fn test() {}
