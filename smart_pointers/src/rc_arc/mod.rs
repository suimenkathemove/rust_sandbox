//! # Rc, Arc
//!
//! 排他制御にもmutable borrowにもリードライトロックという制約がある。

mod arc;
mod rc;

// 仕組み
#[test]
fn structure() {
    use std::rc::Rc;

    // RcやArcで値を生成すると、内部で保持する値とその参照カウントがヒープメモリに確保される
    let a = Rc::new(0);
    // cloneすると参照カウントが1増える
    let _b = Rc::clone(&a);
    // cloneした変数の生存期間が終了すると参照カウントが1減る
    // 参照カウントが0になった時に、ヒープメモリに確保された領域が解放される
}
