use async_trait::async_trait;
use fusion_ibc::component::HostInterface;
use fusion_sct::component::clock::EpochRead;

use crate::app::StateReadExt;

/// The implementation of [`fusion_ibc::component::HostInterface`] for Fusion.
/// It encapsulates all of the chain-specific data that the ibc implementation needs.
#[derive(Clone)]
pub struct FusionHost {}

#[async_trait]
impl HostInterface for FusionHost {
    async fn get_chain_id<S: cnidarium::StateRead>(state: S) -> anyhow::Result<String> {
        state.get_chain_id().await
    }

    async fn get_revision_number<S: cnidarium::StateRead>(state: S) -> anyhow::Result<u64> {
        state.get_revision_number().await
    }

    async fn get_block_height<S: cnidarium::StateRead>(state: S) -> anyhow::Result<u64> {
        state.get_block_height().await
    }

    async fn get_block_timestamp<S: cnidarium::StateRead>(
        state: S,
    ) -> anyhow::Result<tendermint::Time> {
        state.get_current_block_timestamp().await
    }
}
