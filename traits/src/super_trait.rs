//! # super trait
//!
//! ある機能を実現するために別の機能を要求するための仕組み。
//! ```
//! trait Foo: Bar + Baz {}
//! ```
//! だとした場合、BarやBazは、Fooのsuper traitである。
//! 例えばEq traitは、PartialEq traitをsuper traitとして持っている。
