# Rusty Lambdas

This repository is a collection of AWS Lambda functions written in Rust.

## Rust runtime for AWS Lambda

* Runtime: https://github.com/awslabs/aws-lambda-rust-runtime
* About custom runtime: https://docs.aws.amazon.com/lambda/latest/dg/runtimes-custom.html
* Rust support announcement with examples: https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/
* Lambda Runtime crate (outdated): https://crates.io/crates/lambda_runtime

The 2.1 version of the crate is outdated. I use the latest version from GitHub.


## json_logger

This is a very simple function that logs the input passed onto the Lambda into CloudWatch as a JSON string. It comes useful when you need to discover what is being sent from the source.

For example, use it as a handler for an S3 or SNS event to see what payload is being sent with each invocation. Look up the CloudWatch log to see the JSON.

JSON strings can be converted into Rust struct with https://typegen.vestera.as/


