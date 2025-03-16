#![deny(clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
use once_cell::sync::Lazy;

pub mod asset;
pub mod balance;
mod equivalent_value;
mod estimated_price;
mod value;

pub use balance::Balance;
pub use equivalent_value::EquivalentValue;
pub use estimated_price::EstimatedPrice;
pub use value::{Value, ValueVar, ValueView};

pub static STAKING_TOKEN_DENOM: Lazy<asset::Metadata> = Lazy::new(|| {
    asset::Cache::with_known_assets()
        .get_unit("ufusion")
        .expect("unable to get ufusion denom, which should be hardcoded")
        .base()
});
pub static STAKING_TOKEN_ASSET_ID: Lazy<asset::Id> = Lazy::new(|| STAKING_TOKEN_DENOM.id());
