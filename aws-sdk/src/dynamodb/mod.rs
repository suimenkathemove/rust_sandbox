use aws_sdk_dynamodb::{model::AttributeValue, Client, Error};

pub async fn list_table_names() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let res = req.send().await?;
    println!("table_names: {:?}", res.table_names());
    Ok(())
}

pub async fn get_item() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client
        .get_item()
        .table_name("")
        .key("id", AttributeValue::S("".to_string()));
    let res = req.send().await?;
    println!("item: {:?}", res.item());
    Ok(())
}
