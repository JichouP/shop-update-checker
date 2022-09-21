pub mod domain;
pub mod infrastructure;
pub mod service;
pub mod use_case;

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate enum_iterator;

use std::sync::{Arc, Mutex};

use service::{
    context::{Context, Ctx},
    cron::cron,
};
use use_case::shop::check_all_shop_updates;

#[tokio::main]
async fn main() -> Result<(), String> {
    let ctx: Ctx = Arc::new(Mutex::new(Context::new()));

    cron(move || {
        #[allow(unused_must_use)]
        check_all_shop_updates(ctx.clone());
    });

    Ok(())
}
