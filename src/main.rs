pub mod domain;
pub mod infrastructure;

#[macro_use]
extern crate async_trait;

#[tokio::main]
async fn main() -> Result<(), String> {
    println!("Hello, world!");

    Ok(())
}
