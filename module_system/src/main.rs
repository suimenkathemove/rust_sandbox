//! ## crate クレート
//!
//! crateとは、crate rootとその依存ファイル(modules)のこと。
//! crate rootには、binary crateのcrate rootであるmain.rs、library crateのcrate rootであるlib.rsがある。
//! binary crateはそれ単体で実行できる。library crateは他のbinary crateやlibrary crateから利用するためのもの。
//!
//! packageとは、crateとCargo.tomlなどのファイルを含めたもの。
//!
//! moduleとは、crate内でさらに機能を分割したもの。
//!
//! - package
//!   - crate
//!     - crate root
//!     - modules
//!   - Cargo.toml

mod modules;

fn main() {
    println!("Hello, world!");
}
