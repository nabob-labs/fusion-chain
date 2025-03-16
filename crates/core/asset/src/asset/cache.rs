use std::{collections::BTreeMap, ops::Deref, sync::Arc};

use super::{denom_metadata, Id, Metadata, REGISTRY};
use crate::asset::denom_metadata::Unit;

/// On-chain data structures only record a fixed-size [`Id`], so this type
/// allows caching known [`BaseDenom`]s.
///
/// The cache is backed by a [`BTreeMap`] accessed through a [`Deref`] impl.
///
/// For (de)serialization, [`From`] conversions are provided to a `BTreeMap<Id,
/// String>` with the string representations of the base denominations.
#[derive(Clone, Default, Debug)]
pub struct Cache {
    cache: BTreeMap<Id, Metadata>,
    units: BTreeMap<String, Unit>,
}

impl Cache {
    pub fn get_by_id(&self, id: Id) -> Option<Metadata> {
        self.cache.get(&id).cloned()
    }

    pub fn get_unit(&self, raw_denom: &str) -> Option<Unit> {
        self.units.get(raw_denom).cloned()
    }

    pub fn with_known_assets() -> Self {
        let mut cache = Cache::default();

        let known_assets = vec![
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "ufusion".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "fusion".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mfusion".to_string(),
                        },
                    ],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "ugn".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "gn".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mgn".to_string(),
                        },
                    ],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "ugm".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "gm".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mgm".to_string(),
                        },
                    ],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "wtest_usd".to_string(),
                    vec![denom_metadata::BareDenomUnit {
                        exponent: 6,
                        denom: "test_usd".to_string(),
                    }],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "test_sat".to_string(),
                    vec![denom_metadata::BareDenomUnit {
                        exponent: 8,
                        denom: "test_btc".to_string(),
                    }],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "utest_atom".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "test_atom".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mtest_atom".to_string(),
                        },
                    ],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "utest_osmo".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "test_osmo".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mtest_osmo".to_string(),
                        },
                    ],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "uubtc".to_string(),
                    vec![denom_metadata::BareDenomUnit {
                        exponent: 6,
                        denom: "ubtc".to_string(),
                    }],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "ucube".to_string(),
                    vec![denom_metadata::BareDenomUnit {
                        exponent: 1,
                        denom: "cube".to_string(),
                    }],
                )),
            },
            Metadata {
                inner: Arc::new(denom_metadata::Inner::new(
                    "unala".to_string(),
                    vec![
                        denom_metadata::BareDenomUnit {
                            exponent: 6,
                            denom: "nala".to_string(),
                        },
                        denom_metadata::BareDenomUnit {
                            exponent: 3,
                            denom: "mnala".to_string(),
                        },
                    ],
                )),
            },
        ];

        cache.extend(known_assets);

        cache
    }
}

// Implementing Deref but not DerefMut means people get unlimited read access,
// but can only write into the cache through approved methods.
impl Deref for Cache {
    type Target = BTreeMap<Id, Metadata>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

impl From<Cache> for BTreeMap<Id, Metadata> {
    fn from(cache: Cache) -> Self {
        cache
            .cache
            .into_iter()
            .map(|(id, denom)| (id, denom))
            .collect()
    }
}

impl TryFrom<BTreeMap<Id, Metadata>> for Cache {
    type Error = anyhow::Error;

    fn try_from(map: BTreeMap<Id, Metadata>) -> Result<Self, Self::Error> {
        let mut cache = BTreeMap::default();
        let mut units: BTreeMap<String, Unit> = BTreeMap::default();
        for (provided_id, denom) in map.into_iter() {
            if let Some(denom) = REGISTRY.parse_denom(&denom.base_denom().denom) {
                let id = denom.id();
                if provided_id != id {
                    anyhow::bail!(
                        "provided id {} for denom {} does not match computed id {}",
                        provided_id,
                        denom,
                        id
                    );
                }
                cache.insert(id, denom.clone());
                units.insert(denom.base_denom().denom, denom.base_unit());
            } else {
                anyhow::bail!("invalid base denom {}", denom.base_denom().denom);
            }
        }
        Ok(Self { cache, units })
    }
}

// BaseDenom already has a validated Id, so by implementing Extend<BaseDenom> we
// can ensure we don't insert any invalid Ids
impl Extend<Metadata> for Cache {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Metadata>,
    {
        for denom in iter {
            let id = denom.id();
            self.cache.insert(id, denom.clone());

            for unit in denom.units() {
                self.units.insert(unit.to_string(), unit);
            }
        }
    }
}

impl FromIterator<Metadata> for Cache {
    fn from_iter<T: IntoIterator<Item = Metadata>>(iter: T) -> Self {
        let mut cache = Cache::default();
        cache.extend(iter);
        cache
    }
}
