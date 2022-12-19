mod write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write::main()?;

    Ok(())
}
