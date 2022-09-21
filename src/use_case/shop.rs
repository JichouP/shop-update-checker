use crate::{
    domain::shop::{FetchShop, ShopKind},
    infrastructure::fetch::shop::FetchShopImpl,
    service::context::Ctx,
};
use std::sync::Arc;
use tokio_stream::StreamExt;

pub async fn check_shop_update(ctx: Ctx, shop_kind: ShopKind) -> Result<bool, String> {
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

pub async fn check_all_shop_updates(ctx: Ctx) -> Result<Vec<ShopKind>, String> {
    println!("Hello");

    let shop_kind_iter = enum_iterator::all::<ShopKind>();
    let stream = tokio_stream::iter(shop_kind_iter);
    let result = stream
        .then(|shop_kind| async {
            match check_shop_update(ctx.clone(), shop_kind.clone()).await {
                Ok(result) => {
                    if result {
                        Some(shop_kind)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .filter_map(|v| v)
        .collect::<Vec<_>>()
        .await;

    println!("{:?}", result);

    Ok(result)
}
