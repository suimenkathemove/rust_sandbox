// クロージャの概念的な構造
fn _closure_struct() {
    // クロージャの環境
    struct Env<'a, T> {
        variable: &'a mut T,
    }

    // クロージャのファットポインタ
    struct Closure<'a, T> {
        ptr_func: fn(),
        // キャプチャした変数を保持する領域を`環境`と呼ぶ
        ptr_env: Box<Env<'a, T>>,
    }
}

pub fn main() {
    {
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

    {
        let mut n = 1;
        // 環境に変更を加えるため、変数fもmutにしなければならない
        let mut f = || {
            n += 1;
        };
        f();
    }
}
