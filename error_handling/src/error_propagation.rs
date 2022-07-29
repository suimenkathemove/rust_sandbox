use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
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

pub fn main() {
    exec_func(2).ok();
    println!("{}", exec_func(1).err().unwrap());
}
