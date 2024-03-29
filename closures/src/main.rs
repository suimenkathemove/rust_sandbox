//! # Closures
//!
//! キャプチャした変数を保持する領域を`環境`と呼ぶ
//!
//! 関数は環境（定義されたスコープ）の値をキャプチャできないが、クロージャはできる。環境から値をキャプチャするとメモリを使用してその値を保存するが、オーバーヘッドになる
//!
//! プログラムの1箇所でコードを定義したいが、結果が必要なところでだけコードを実行したい場合に使う
//!
//! 狭いコンテキストでしか使われないので
//!
//! - 環境の値をキャプチャできる
//! - 引数や戻り値の型を注釈する必要がない
//!
//! シグニチャが同じでも異なる型として扱う
//!
//! ## Fn、FnMut、FnOnce
//!
//! 全てのクロージャと関数は、`Fn`、`FnMut`、`FnOnce`のいずれか1つ以上を実装している
//!
//! キャプチャした環境の値の所有権を奪うことを強制する場合は`move`を使う。クロージャが定義された際にムーブされる

mod cacher;
mod capture_env;
mod move_;
mod recursive;

fn main() {
    cacher::main();
    capture_env::main();
    move_::main();
    recursive::main();
}
