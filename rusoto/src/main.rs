use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ScanInput, UpdateItemInput};
use std::collections::HashMap;

async fn remove_attributes(table_name: &str, attribute_names: Vec<&str>) -> anyhow::Result<()> {
    let client = DynamoDbClient::new(Region::ApNortheast1);

    let items = client
        .scan(ScanInput {
            table_name: table_name.to_string(),
            ..Default::default()
        })
        .await?
        .items
        .expect("failed to get items");
    for item in items.iter() {
        let id = item.get("id").expect("failed to get id");
        client
            .update_item(UpdateItemInput {
                table_name: table_name.to_string(),
                key: HashMap::from([("id".to_string(), id.clone())]),
                update_expression: Some(format!("REMOVE {}", attribute_names.join(", "))),
                ..Default::default()
            })
            .await?;
    }

    Ok(())
}

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
