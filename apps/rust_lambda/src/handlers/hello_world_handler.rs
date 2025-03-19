use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
struct Request {
    name: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    request_id: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Value, Error> {
    // Extract some useful info from the request
    let name = event.payload.name;
    let request_id = event.context.request_id;

    // Prepare the response
    let resp = Response {
        message: format!("Hello, {}!", name),
        request_id,
    };

    // Return a response as a JSON object
    Ok(json!(resp))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS Lambda runtime
    let func = service_fn(function_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}