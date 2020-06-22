use serde_json::Value;

/// This function logs the input JSON in CloudWatch log as a string.
/// `event.Value` can be replaced with a struct the JSON deserializes into.
pub(crate) async fn func(event: Value) -> Result<Value, crate::Error> {
    let event = event.to_string();
    log::info!("Input JSON: {}", event);
    Ok(Value::Null)
}
