use aws_sdk::dynamodb::list_table_names;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    list_table_names().await?;

    Ok(())
}
