use std::fmt::Display;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Sequence)]
pub enum ShopKind {
    Ark,
    PCKoubou,
    Sofmap,
    Tsukumo,
}

impl Display for ShopKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ark => {
                write!(f, "Ark")
            }
            Self::PCKoubou => {
                write!(f, "PCKoubou")
            }
            Self::Sofmap => {
                write!(f, "Sofmap")
            }
            Self::Tsukumo => {
                write!(f, "Tsukumo")
            }
        }
    }
}

#[async_trait]
pub trait FetchShop {
    async fn fetch(&mut self, shop_kind: ShopKind) -> Result<String, String>;
}
