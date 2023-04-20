use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = DynamoDbClient::new(Region::ApNortheast1);
    let output = client.list_tables(Default::default()).await?;
    match output.table_names {
        Some(table_names) => {
            table_names.iter().for_each(|table_name| {
                dbg!(table_name);
            });
        }
        None => {
            println!("no tables");
        }
    }

    Ok(())
}
