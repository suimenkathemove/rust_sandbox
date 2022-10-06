//! # 単体テスト
//!
//! `/src`に置く
//!
//! `#cfg(test)`で注釈された`tests`モジュールをテスト対象のファイルに置く
//!
//! `#cfg(test)`で注釈するとプロダクションコードから削除される

fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_two_is_three() {
        assert_eq!(3, add(1, 2));
    }
}
