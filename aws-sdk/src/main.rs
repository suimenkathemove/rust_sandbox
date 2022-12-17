mod dynamodb;

use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    dynamodb::list_table_names().await?;

    Ok(())
}
