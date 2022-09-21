use std::collections::HashMap;

use crate::domain::{
    shop::FetchShop,
    shop_url::{generate_shop_urls, ShopUrlMap},
};

pub struct FetchShopImpl {
    urls: ShopUrlMap,
}

impl FetchShopImpl {
    pub fn new() -> Self {
        Self {
            urls: HashMap::new(),
        }
    }

    fn update_url(&mut self) {
        self.urls = generate_shop_urls();
    }
}

#[async_trait]
impl FetchShop for FetchShopImpl {
    async fn fetch(&mut self, shop_kind: crate::domain::shop::ShopKind) -> Result<String, String> {
        self.update_url();

        let url = self.urls.get(&shop_kind).ok_or_else(|| format!(""))?;

        let response = reqwest::get(url).await.map_err(|e| e.to_string())?;
        let response_bytes = response.bytes().await.map_err(|e| e.to_string())?.to_vec();
        let response_string = String::from_utf8(response_bytes).map_err(|e| e.to_string())?;

        Ok(response_string)
    }
}
