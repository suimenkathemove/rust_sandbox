mod markdown_to_rust_doc_comments;
mod write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write::main()?;

    Ok(())
}
