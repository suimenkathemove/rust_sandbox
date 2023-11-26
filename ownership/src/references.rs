//! # References
//!
//! ## 参照とポインタ
//!
//! 参照はライフタイムによって不正なアドレスを指すことはないが、ポインタは不正なアドレスを指すことがある。
//! 参照は、安全で、オーバーヘッドが小さい。
//!
//! ## 借用
//!
//! 借用とは、変数の参照をとること。
//!
//! 可変参照のルール
//! - 借用中に参照先の変数を変更できない
//! - ある変数を可変借用している状態でその変数を借用(不変、可変を問わない)した場合、最後にとった参照のみ使用できる
//!
//! 不変借用はコピーセマンティクスで、可変借用はムーブセマンティクスである。
//!
//! データ競合は以下の条件がすべて満たされている場合に発生する。
//! - 2つ以上のポインタが同じデータに同時にアクセスする
//! - 1つ以上のポインタがデータに書き込みを行なっている
//! - データへのアクセスを同期する機構が使用されていない

#[cfg(test)]
mod mutable_borrow_rules {
    // 借用中に参照先の変数を変更できない
    #[test]
    fn cannot_change_target_while_borrowing() {
        let mut a = 0;
        let b = &mut a;
        a = 1;
        // println!("{}", b); // error
    }

    // 可変参照は1つだけ
    #[test]
    fn mutable_and_mutable() {
        let mut a = 0;
        let b = &mut a;
        let c = &mut a;
        // 最後の参照のみ使用できる
        println!("{}", c);
        // cがdropされてもbは使用できない
        // println!("{}", b); // error
    }

    // 可変参照と不変参照のどちらかだけ
    #[test]
    fn mutable_and_immutable() {
        let mut a = 0;
        let b = &mut a;
        let c = &a;
        // 最後の参照のみ使用できる
        println!("{}", c);
        // cがdropされてもbは使用できない
        // println!("{}", b); // error
    }
}

#[cfg(test)]
mod borrow_borrow {
    // 不変参照を不変借用
    // 何も特別なことはない
    #[test]
    fn immutable_immutable() {
        let a = 0;
        let b = &a;
        let c = &b;
        println!("{}, {}, {}", a, b, c);
    }

    // 可変参照を不変借用
    // 不変参照がdropされるまで書き込みができなくなるが、読み込みはできる
    #[test]
    fn immutable_mutable() {
        let mut a = 0;
        let b = &mut a;
        let c = &b;
        // cがdropされるまで書き込みができない
        // *b = 1; // error
        // 読み込みはできる
        println!("{}, {}", b, c);
        *b = 1;
    }

    // 不変参照を可変借用
    // 可変参照がdropされるまで使用できない
    #[test]
    fn mutable_immutable() {
        let a = 0;
        let mut b = &a;
        let c = &mut b;
        // cがdropされるまでbは使用できない
        // println!("{}, {}", b, c); // error
    }

    // 可変参照を可変借用
    // 最後の可変参照がdropされるまで最初の可変参照は使用できない
    #[test]
    fn mutable_mutable() {
        let mut a = 0;
        let mut b = &mut a;
        let c = &mut b;
        // cがdropされるまでbは使用できない
        // println!("{}, {}", b, c); // error
        *b = 1;
    }
}

#[cfg(test)]
mod semantics {
    // 不変借用はコピーセマンティクス
    #[test]
    fn immutable_is_copy() {
        let a = 0;
        let b = &a;
        let c = b;
        println!("{}, {}", b, c);
    }

    // 可変借用はムーブセマンティクス
    #[test]
    fn mutable_is_move() {
        let mut a = 0;
        let b = &mut a;
        let c = b;
        // println!("{}, {}", b, c); // error
    }

    // 可変借用はムーブセマンティクスだが、関数の呼び出し時にはムーブされない
    #[test]
    fn not_moved_when_function_calling() {
        let mut a = 0;
        let b = &mut a;
        fn f(_: &mut i32) {}
        f(b);
        // ムーブされていないので使用できる
        *b = 1;
    }
}
