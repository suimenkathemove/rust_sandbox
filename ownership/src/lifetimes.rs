//! # Lifetimes
//!
//! ライフタイムは、変数の有効なスコープのことで、参照を使用する際に考慮する必要がある。
//! これは、ダングリング参照を回避するためのもの。
//! Rustコンパイラは、コンパイル時に変数のライフタイムを計算して整合性を検査する。
//! 参照先のライフタイムが、参照元より長く覆われていなければならない。
//!
//! ライフタイムには、字句ライフタイムと非字句ライフタイムがある。
//! 字句ライフタイムでは、ライフタイムは変数が定義されてからその変数が定義されたブロックの終了までとする。単純に字句の並びから決定する。
//! 非字句ライフタイムでは、変数の利用のされ方まで、意味的な解釈をしてライフタイムが決定される。
//!
//! ## ライフタイム注釈
//!
//! ライフタイム注釈は、複数の参照のジェネリックなライフタイム引数が互いにどう関係するかを示すもの。
//! ライフタイム注釈は、ライフタイムを変えるわけではない。
//! 関数や構造体に指定する。
//!
//! ### 関数
//!
//! 引数に関数外からの参照や、戻り値に関数外への参照がある場合は、Rustコンパイラがそれらのライフタイムを自力で解決することがほぼできなくなる。
//! そのライフタイムは関数を呼び出す度に異なる可能性があるため、手動でライフタイムを注釈する必要がある可能性がある。
//!
//! 関数から参照を返す場合、戻り値型のライフタイム引数は、引数のどれかに一致させる必要がある。
//!
//! ### 構造体
//!
//! 構造体に参照を保持させる場合にライフタイム注釈が必要となり、これは省略できない。
//!
//! 構造体のインスタンスはフィールドの参照よりも長生きできない。
//!
//! ### ライフタイム省略規則(ライフタイム注釈を省略した場合は以下の挙動になる)
//!
//! - 参照の各引数は独自のライフタイム引数を得る。
//! - 入力ライフタイム引数が1つの場合は、出力ライフタイム引数と同じになる。
//! - メソッドの場合で`self`がある場合は、`self`のライフタイムが出力ライフタイム引数に代入される。
//!
//! ### 静的ライフタイム
//!
//! 静的ライフタイムは、プログラムの開始から終了までのライフタイムであり、`'static`と書く。
//!
//! 文字列リテラルが静的ライフタイムを持つ。

#[cfg(test)]
mod basic {
    // aのライフタイムは1~6行目までであり、bのライフタイムは3~5行目までである
    // bのライフタイムがaより短いため、エラーになる
    #[test]
    fn error() {
        let a;
        {
            let b = 10;
            a = &b;
        }
        // println!("{}", a);
    }

    // aの字句ライフタイムは1~6行目だが、非字句ライフタイムはaが最後に利用される5行目までなので、ライフタイムは1~5行目になる
    // aのライフタイムの終了がbと同じため、パスする
    #[test]
    fn pass() {
        let a;
        {
            let b = 10;
            a = &b;
            println!("{}", a);
        }
    }
}

#[cfg(test)]
mod func {
    #[test]
    fn same_lifetimes() {
        // 引数で同じライフタイムを使用している場合、xとyのライフタイムのうち小さい方が'aに代入される
        fn f<'a>(x: &'a str, y: &'a str) -> &'a str {
            ""
        }

        let result;
        let s1 = "1".to_string();
        {
            let s2 = "2".to_string();
            // s1とs2のライフタイムではs2の方が小さいため、'aにs2のライフタイムが代入される
            result = f(&s1, &s2);
        }
        // resultのライフタイムはs2と同じになるのでエラー
        // println!("{}", result);
    }

    #[test]
    fn different_lifetimes() {
        fn f<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
            ""
        }

        let result;
        let s1 = "1".to_string();
        {
            let s2 = "2".to_string();
            // 'aにs1のライフタイムが、'bにs2のライフタイムが代入される
            result = f(&s1, &s2);
        }
        // resultのライフタイムはs1と同じになるのでパス
        println!("{}", result);
    }

    // 引数が1つだけの場合はライフタイム注釈は必要ない
    fn only_one_arg(s: &str) -> &str {
        s
    }
}

#[cfg(test)]
mod _struct {
    struct S<'a> {
        s: &'a str,
    }

    impl<'a> S<'a> {
        // ライフタイム注釈を省略した場合、selfのライフタイムが出力ライフタイム引数に代入される
        fn method(&self, s: &str) -> &str {
            self.s
        }

        // 引数をそのまま返す場合はライフタイム注釈が必要になる
        fn method2<'b>(&self, s: &'b str) -> &'b str {
            s
        }
    }
}
