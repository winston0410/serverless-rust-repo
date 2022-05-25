use lambda_http::{run, service_fn, Error, IntoResponse, Request};
use serde_json::{json};

async fn handler(_event: Request) -> Result<impl IntoResponse, Error> {
    Ok(json!({
        "message": "Bye!"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Disable tracing for maximum raw speed test
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}