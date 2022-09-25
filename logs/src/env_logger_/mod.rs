//! # env_logger
//!
//! `log` クレートの `logging facade` と共に使う
//!
//! 環境変数を通して設定できるが、環境変数以外の方法でも設定できる
//!
//! デフォルトでは `stderr` にログを書き込むが、 `stdout` に書き込むこともできる
//!
//! ```
//! $ RUST_LOG=debug cargo run
//! $ RUST_LOG=info cargo run
//! $ RUST_LOG=error cargo run
//! ```
//!
//! ログレベルはモジュールごとに設定できる
//! ```
//! $ RUST_LOG=main=error cargo run
//! ```
//!
//! ## progress
//!
//! ### https://docs.rs/env_logger/latest/env_logger/
//!
//! - [x] Example

use log::{debug, error, info};

pub fn main() {
    env_logger::init();

    debug!("debug");
    info!("info");
    error!("error");
}
