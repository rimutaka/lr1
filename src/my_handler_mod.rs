use lambda_runtime::{error::HandlerError, Context};
use log::{self, error};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;

#[derive(Deserialize)]
pub struct CustomEvent {
  #[serde(rename = "firstName")]
  pub first_name: String,
}

#[derive(Serialize)]
pub struct CustomOutput {
  pub message: String,
}

pub fn my_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
  if e.first_name == "" {
    error!("Empty first name in request {}", c.aws_request_id);
    bail!("Empty first name");
  }

  Ok(CustomOutput {
    message: format!("Hello, {}!", e.first_name),
  })
}

#[cfg(test)]
mod my_handler_test {

  use super::{my_handler, CustomEvent};
  use lambda_runtime::Context;

  #[test]
  fn invoke_my_handler() {
    let ce = CustomEvent {
      first_name: String::from("Jimmy"),
    };
    let ctx = Context {
      memory_limit_in_mb: 512,
      function_name: String::from("My_Test_Lambda"),
      function_version: String::from("1"),
      invoked_function_arn: String::from("1"),
      aws_request_id: String::from("1"),
      xray_trace_id: Option::None,
      log_stream_name: String::from("1"),
      log_group_name: String::from("1"),
      client_context: Option::None,
      identity: Option::None,
      deadline: 100000,
    };

    let co = my_handler(ce, ctx).expect("my_handler failed");
    println!("output: {}",co.message);

    assert_eq!(co.message, "Hello, Jimmy!");
  }}
