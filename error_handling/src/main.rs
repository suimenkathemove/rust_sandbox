use thiserror::Error;

// エラーの内容を表示できるようになる
#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivByZero(i32),
}

fn div(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        return Err(DivError::DivByZero(x));
    }

    Ok(x / y)
}

fn display_result<T: std::fmt::Display, E: std::fmt::Display>(result: Result<T, E>) {
    match result {
        Ok(v) => {
            println!("{}", v);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn main() {
    // matchで場合分けする

    // 起こり得るエラーの種類を網羅した列挙型を定義する

    match div(2, 0) {
        Ok(v) => {
            println!("{}", v);
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    let some_v = div(2, 2).ok();
    let some_e = div(2, 0).err();
}
