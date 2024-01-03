fn sum(first: i32, second: i32) -> i32 {
    first + second
}

// テストスイート
#[cfg(test)]
mod tests {
    use super::*;

    // テストケース
    #[test]
    fn sum_of_two_numbers() {
        // 準備(Arrange)
        let first = 10;
        let second = 20;

        // 実行(Act)
        let result = sum(first, second);

        // 確認(Assert)
        assert_eq!(30, result);
    }
}
