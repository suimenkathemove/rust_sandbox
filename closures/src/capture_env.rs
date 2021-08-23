pub fn main() {
    let n = 0;

    // 関数は環境をキャプチャできない
    // fn function() -> i32 {
    //     n
    // }
    // function();

    // クロージャは環境をキャプチャできる
    let closure = || n;
    closure();
}
