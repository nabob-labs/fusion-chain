//! Hardcoded test keys used by the `Default` genesis state and test code.

use once_cell::sync::Lazy;

use crate::{
    keys::{Bip44Path, SpendKey, WalletId},
    Address, FullViewingKey,
};

/// This address is for test purposes, allocations were added beginning with
/// the 062-Iapetus testnet.
/// Previously the test data was generated using BIP39 derivation starting with
/// the 016-Pandia testnet.
pub const SEED_PHRASE: &str = "comfort ten front cycle churn burger oak absent rice ice urge result art couple benefit cabbage frequent obscure hurry trick segment cool job debate";

/// These addresses both correspond to the test wallet above.
pub const ADDRESS_0_STR: &str = "fusion147mfall0zr6am5r45qkwht7xqqrdsp50czde7empv7yq2nk3z8yyfh9k9520ddgswkmzar22vhz9dwtuem7uxw0qytfpv7lk3q9dp8ccaw2fn5c838rfackazmgf3ahh09cxmz";
/// These addresses both correspond to the test wallet above.
pub const ADDRESS_1_STR: &str = "fusion1vmmz304hjlkjq6xv4al5dqumvgk3ek82rneagj07vdqkudjvl6y7zxzr5k6qq24yc7yyyekpu9qm7ef3acg2u8p950hs6hu3e73guq5pfmmvm63qudfx4qmg8h7fdweyw3ektn";

pub static ADDRESS_0: Lazy<Address> = Lazy::new(|| {
    ADDRESS_0_STR
        .parse()
        .expect("hardcoded test addresses should be valid")
});
pub static ADDRESS_1: Lazy<Address> = Lazy::new(|| {
    ADDRESS_1_STR
        .parse()
        .expect("hardcoded test addresses should be valid")
});

/// The test account's spend key.
pub static SPEND_KEY: Lazy<SpendKey> = Lazy::new(|| {
    SpendKey::from_seed_phrase_bip44(
        SEED_PHRASE
            .parse()
            .expect("hardcoded test seed phrase should be valid"),
        &Bip44Path::new(0),
    )
});

/// The test account's full viewing key, as a string.
pub const FULL_VIEWING_KEY_STR: &str = "fusionfullviewingkey1vzfytwlvq067g2kz095vn7sgcft47hga40atrg5zu2crskm6tyyjysm28qg5nth2fqmdf5n0q530jreumjlsrcxjwtfv6zdmfpe5kqsa5lg09";

/// The test account's full viewing key.
pub static FULL_VIEWING_KEY: Lazy<FullViewingKey> = Lazy::new(|| {
    FULL_VIEWING_KEY_STR
        .parse()
        .expect("hardcoded test fvk should be valid")
});

pub static WALLET_ID: Lazy<WalletId> = Lazy::new(|| FULL_VIEWING_KEY.wallet_id());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fvk_matches() {
        assert_eq!(*FULL_VIEWING_KEY, *SPEND_KEY.full_viewing_key());
    }
}
