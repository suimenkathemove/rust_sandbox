//! Generics
//!
//! ジェネリクス（generics）には、ジェネリック型（generic type）とジェネリック関数（generic function）がある。
//! ジェネリック型は、定数や型を受け取り、型を返す。
//! ジェネリック関数は、定数や型を受け取り、関数を返す。

#![allow(dead_code)]

mod const_;
mod point;

fn main() {
    const_::main();
    point::main();
}
