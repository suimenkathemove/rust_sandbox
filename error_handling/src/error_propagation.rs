use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error")]
    Variant(i32),
}

fn func(arg: i32) -> Result<i32, Error> {
    if arg % 2 == 0 {
        Ok(arg)
    } else {
        Err(Error::Variant(arg))
    }
}

fn exec_func(arg: i32) -> Result<(), Error> {
    println!("result: {}", func(arg)?);
    Ok(())
}

// std::error::Errorトレイトを満たす型　トレイトオブジェクト
fn multiple_error(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let i = s.parse::<i32>()?;
    Ok(func(i)?)
}

pub fn main() -> Result<i32, Error> {
    exec_func(2).ok();
    println!("{}", exec_func(1).err().unwrap());

    Ok(func(1)?)
}

#[cfg(test)]
mod tests {
    enum Error1 {
        Foo,
    }

    enum Error2 {
        Foo,
    }

    impl From<Error1> for Error2 {
        fn from(error: Error1) -> Self {
            match error {
                Error1::Foo => Error2::Foo,
            }
        }
    }

    fn error1() -> Result<(), Error1> {
        Err(Error1::Foo)
    }

    fn error2() -> Result<(), Error2> {
        // ?を使うとfromを呼び出してくれる
        Ok(error1()?)
    }

    #[test]
    fn test() {
        let _ = error2();
    }
}
