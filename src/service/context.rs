use crate::infrastructure::store::shop_store::ShopStore;
use std::sync::{Arc, Mutex};

pub type Ctx = Arc<Mutex<Context>>;

pub struct Context {
    pub store: ShopStore,
}

impl Context {
    pub fn new() -> Self {
        Self {
            store: ShopStore::new(),
        }
    }
}
