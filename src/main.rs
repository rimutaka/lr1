use std::error::Error;

use lambda_runtime::{lambda};
use simple_logger;

mod my_handler_mod;


fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug)?;
    lambda!(my_handler_mod::my_handler);

    Ok(())
}


