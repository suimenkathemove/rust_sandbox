//! # Copy, Clone
//!
//! ## Copy
//!
//! Copyトレイトは、ビット単位のコピー(bit-wise copy)をする。
//!
//! Copyトレイトを実装するには、Cloneトレイトも実装する必要がある。
//! 各フィールドもCopyトレイトを実装する必要がある。
//!
//! ## Clone
//!
//! Cloneトレイトは、値をディープコピーする。
//! Copyトレイトで可能なことはCloneトレイトでも可能だが、パフォーマンスが低下する可能性がある。
//!
//! 各フィールドもCloneトレイトを実装する必要がある。
