//! # BTreeMap
//!
//! 内部的には木構造を使っている。
//! 木構造のため、キーの順序を保持できる。
//! 挿入・削除・検索の計算量はO(log n)であり、エントリ数が数千や数万でもない限りHashMapと差はほとんどない。