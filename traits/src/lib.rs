//! # traits
//!
//! トレイトとは、インターフェースのようなものである。
//!
//! トレイトか対象の型がクレート固有の場合のみ、そのトレイトをその型に実装できる。
//! 外部のトレイトを外部の型に実装することはできない。

mod associated_types;
mod derive;
mod existential_type;
mod super_trait;
mod trait_object;
