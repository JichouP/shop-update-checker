use crate::domain::shop::ShopKind;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub struct ShopStore {
    store: HashMap<ShopKind, String>,
}

impl ShopStore {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, shop_kind: &ShopKind) -> Option<&String> {
        self.store.get(shop_kind)
    }

    pub fn insert(&mut self, shop_kind: ShopKind, content: String) {
        self.store.insert(shop_kind, content);
    }

    pub fn eq_or_insert(&mut self, shop_kind: ShopKind, content: String) -> Result<bool, String> {
        let Some(result) = self.store.get(&shop_kind) else {
            return Err(format!("Value of {} is not set to ShopStore", shop_kind)) 
        };

        if &content == result {
            return Ok(true);
        }

        self.insert(shop_kind, content);

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> ShopStore {
        let mut store = ShopStore::new();
        store.insert(ShopKind::Ark, "Content".to_string());
        store
    }

    #[test]
    fn test_init() {}

    #[test]
    fn test_eq_true() {
        let mut store = init();

        let result = store.eq_or_insert(ShopKind::Ark, "Content".to_string());
        assert_eq!(result, Ok(true));
    }

    #[test]
    fn test_eq_false() {
        let mut store = init();

        let result = store.eq_or_insert(ShopKind::Ark, "NewContent".to_string());
        assert_eq!(result, Ok(false));
    }

    #[test]
    fn test_eq_not_found_error() {
        let mut store = init();

        let result = store.eq_or_insert(ShopKind::PCKoubou, "Content".to_string());
        assert_eq!(
            result,
            Err(format!("Value of PCKoubou is not set to ShopStore"))
        );
    }
}
