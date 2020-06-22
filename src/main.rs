/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info
/// on Rust runtime for AWS Lambda
use lambda::handler_fn;
use simple_logger;
use tokio;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod my_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Debug)?;
    let func = handler_fn(my_handler::func);
    lambda::run(func).await?;
    Ok(())
}


