use crate::infrastructure::store::shop_store::ShopStore;

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
