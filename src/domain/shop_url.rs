use super::shop::ShopKind;
use chrono::{FixedOffset, TimeZone, Utc};
use std::collections::HashMap;

pub fn generate_shop_urls() -> HashMap<ShopKind, String> {
    let current_date = FixedOffset::east(3600 * 9)
        .from_utc_datetime(&Utc::now().naive_utc())
        .format("%Y%m%d%H%M%S")
        .to_string();
    let current_millis = Utc::now().timestamp_millis().to_string();
    let pc_koubou_url = format!("https://www.pc-koubou.jp/search/npsearch.php?time={}fmt=json&searchbox=1&q=RTX+4090&n11l=&n11h=&sort=Score&s5[]=&s2b=%E9%80%9A%E5%B8%B8&s1=1&limit=20&callback=xsearchCallback&fmt=jsonp&s2b=%E9%80%9A%E5%B8%B8&_={}", current_date, current_millis);

    HashMap::from([
        (
            ShopKind::Ark,
            "https://www.ark-pc.co.jp/search/?key=RTX+4090&only_body=1&only_chk=price".to_string(),
        ),
        (ShopKind::PCKoubou, pc_koubou_url),
        (
            ShopKind::Sofmap,
            "https://www.sofmap.com/search_result_json.aspx?gid=001030&keyword=RTX+4090&type=left"
                .to_string(),
        ),
        (
            ShopKind::Tsukumo,
            "https://shop.tsukumo.co.jp/search?keyword=RTX+4090".to_string(),
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let urls = generate_shop_urls();
        assert_eq!(urls.keys().len(), 4);
        assert_eq!(
            urls.get(&ShopKind::Ark),
            Some(
                &"https://www.ark-pc.co.jp/search/?key=RTX+4090&only_body=1&only_chk=price"
                    .to_string()
            )
        );
    }
}
