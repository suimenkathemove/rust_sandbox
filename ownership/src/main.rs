//! Ownership
//!
//! 所有権によってメモリを安全に効率的に利用できる
//!
//! ヒープデータを管理するためのもの
//!
//! 全てのプログラムは、実行中にメモリの使用方法を管理する必要がある
//!
//! Rustでは、メモリはコンパイル時にチェックする一定の規則と所有権システムによって管理されている
//!
//! Rustのようなシステムプログラミング言語では、データがスタックに積まれるか、ヒープに置かれるかで振る舞いが変わる
//!
//! ## 束縛
//!
//! 1. メモリ領域を確保する
//! 2. そのメモリ領域に値をセットする
//! 3. その値に変数のラベルをする
//!
//! ## スタック
//!
//! スタックは皿を重ねたようなもの
//!
//! データにアクセスする場所は常に1番上なので高速
//!
//! データは既知の固定サイズでなければならない
//!
//! 関数の呼び出し時に、引数や関数のローカル変数の値がスタックに積まれ、実行終了時に取り除かれる
//!
//! ## ヒープ
//!
//! メモリを確保する⋯ヒープにデータを置く際に、OSは十分な大きさの空の領域を見つけ、使用中にし、ポインタを返す
//!
//! そのポインタは既知の固定サイズなのでスタックに積むことができる
//!
//! ポインタを追ってデータにアクセスする必要があることや、メモリ確保に時間がかかることがあるので、スタックよりも低速
//!
//! 所有権が以下を解決
//!
//! - どの部分のコードがヒープ上のどのデータを使用しているか把握すること
//! - ヒープ上の重複するデータを最小化すること
//! - ヒープ上の未使用のデータを掃除すること
//!
//! ## 所有権規則
//!
//! - 変数は値の所有者であり、所有者は一つ
//! - 所有者がスコープから外れたら値は破棄される
//!
//! メモリは実行時にOSに要求され、値を使用し終わったらOSにメモリを返還する必要がある
//!
//! 閉じ波括弧で`drop`関数が呼び出される
//!
//! コレクションのデータは3つの要素で構成されている
//!
//! - ptr
//! - len
//! - capacity
//!
//! ## ムーブ
//!
//! スタックのデータのみをコピーし、借用元の変数を無効化する
//!
//! 変数の代入時や関数に値を渡すときに発生する
//!
//! 関数から値を返すと所有権は移動する
//!
//! ## クローン
//!
//! ヒープ上のデータもコピーする
//!
//! ## コピー
//!
//! スタックのみのデータのコピー
//!
//! `Copy` トレイトを実装している場合に、変数の代入時や関数に値を渡すときに発生する
//!
//! 型やその一部分でも `Drop` トレイトを実装している場合、 `Copy` トレイトによる注釈ができなくなる

mod lifetimes;
mod references;

fn main() {
    lifetimes::main();
    references::main();
}
