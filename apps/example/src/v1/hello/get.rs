// use aws_config::meta::region::RegionProviderChain;
use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use serde_json::{json};
// use aws_sdk_dynamodb::model::AttributeValue;

async fn handler(_event: Request) -> Result<impl IntoResponse, Error> {
    // let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    // let config = aws_config::from_env().region(region_provider).load().await;
    // let client = aws_sdk_dynamodb::Client::new(&config);

    Ok(json!({
        "message": "Hello!"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}