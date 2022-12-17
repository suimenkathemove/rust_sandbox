use aws_sdk_dynamodb::{Client, Error};
use dotenv::dotenv;

pub async fn list_table_names() -> Result<(), Error> {
    dotenv().ok();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let res = req.send().await?;
    println!("table_names: {:?}", res.table_names());
    Ok(())
}
