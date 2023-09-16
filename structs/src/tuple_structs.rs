//! # タプル構造体
//!
//! タプルに名前を付けたいが、構造体にするとフィールドに紐づけられた名前は必要ない場合に使う
//!
//! 基本はタプルと同じ

struct Color(i32, i32, i32);

struct Position(i32, i32, i32);

fn func() {
    let mut black = Color(0, 0, 0);

    // 同じ値だが、型が違うのでエラー
    // black = Position(0, 0, 0);
}
