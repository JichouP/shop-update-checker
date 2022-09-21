use crate::{
    domain::shop::{FetchShop, ShopKind},
    infrastructure::fetch::shop::FetchShopImpl,
    service::context::Context,
};
use std::sync::{Arc, Mutex};

pub async fn check_shop_update(
    ctx: Arc<Mutex<Context>>,
    shop_kind: ShopKind,
) -> Result<bool, String> {
    let ctx = Arc::clone(&ctx);

    let mut fetch_shop = FetchShopImpl::new();

    let fetch_result = fetch_shop.fetch(shop_kind.clone()).await?;

    let is_eq = ctx
        .lock()
        .unwrap()
        .store
        .eq_or_insert(shop_kind, fetch_result)?;

    let is_updated = !is_eq;

    Ok(is_updated)
}
