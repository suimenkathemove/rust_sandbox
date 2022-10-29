use std::io::Write;

pub fn main() -> std::io::Result<()> {
    let mut w = Vec::new();
    write!(&mut w, "a")?;
    write!(&mut w, "b{}", "c")?;

    assert_eq!(w, b"abc");

    Ok(())
}
