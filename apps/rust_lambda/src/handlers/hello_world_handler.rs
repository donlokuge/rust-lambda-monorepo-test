use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

    let resp = Response {
        message: format!("Hello, {}!", name),
    };

    Ok(json!(resp))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS Lambda runtime
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}
