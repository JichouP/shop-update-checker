pub mod domain;
pub mod infrastructure;
pub mod service;
pub mod use_case;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Hello, world!");

    Ok(())
}
