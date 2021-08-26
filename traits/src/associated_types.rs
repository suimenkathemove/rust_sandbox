//! # 関連型
//!
//! プレースホルダーの型
//!
//! ジェネリクスと異なり、一回しか実装できない。ジェネリクスだと、実装ごとに具体的な型を指定する必要があり、複数回実装できる

trait AssociatedTypeTrait {
    type T;

    fn func() -> Self::T;
}

trait GenericTrait<T> {
    fn func() -> T;
}

struct Struct;

impl AssociatedTypeTrait for Struct {
    type T = i32;

    fn func() -> Self::T {
        0
    }
}

// error
// impl AssociatedTypeTrait for Struct {
//     type T = f64;

//     fn func() -> Self::T {
//         0.
//     }
// }

impl GenericTrait<i32> for Struct {
    fn func() -> i32 {
        0
    }
}

impl GenericTrait<f64> for Struct {
    fn func() -> f64 {
        0.
    }
}

pub fn main() {}
