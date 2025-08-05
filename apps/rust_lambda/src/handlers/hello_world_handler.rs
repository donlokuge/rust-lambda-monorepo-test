use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use aws_config::load_from_env;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use tracing::{info, error};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Value, Error> {
    let name = event.payload.name;
    let table_name = match std::env::var("DYNAMO_TABLE") {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to read DYNAMO_TABLE env var: {}", e);
            return Err(e.into());
        }
    };

    let config = load_from_env().await;
    let client = Client::new(&config);

    // Attempt to write to DynamoDB
    if let Err(e) = client
        .put_item()
        .table_name(&table_name)
        .item("name", AttributeValue::S(name.clone()))
        .item("timestamp", AttributeValue::S(chrono::Utc::now().to_rfc3339()))
        .send()
        .await
    {
        error!("Failed to write to DynamoDB: {}", e);
        return Err(e.into());
    }

    info!("Successfully wrote to DynamoDB for name: {}", name);

    let resp = Response {
        message: format!("Hello, {}!", name),
    };

    Ok(json!(resp))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}