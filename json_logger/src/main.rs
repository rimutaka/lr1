/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info
/// on Rust runtime for AWS Lambda
use lambda::handler_fn;
use serde_json::Value;
use simple_logger;
use tokio;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info)?;
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

/// This function logs the input JSON in CloudWatch log as a string.
/// `event.Value` can be replaced with a struct the JSON deserializes into.
pub(crate) async fn func(event: Value, _ctx: lambda::Context) -> Result<Value, crate::Error> {
    let event = event.to_string();
    log::info!("Input JSON: {}", event);
    Ok(Value::Null)
}
