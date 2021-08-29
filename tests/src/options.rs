// 成功時もメッセージを出力したい場合は`--nocapture`フラグを使用する
#[test]
fn ok() {
    println!("成功しました");
}

#[test]
fn fail() {
    println!("失敗します");

    panic!();
}
