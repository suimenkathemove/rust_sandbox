mod dynamodb;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dynamodb::list_table_names().await?;
    Ok(())
}
