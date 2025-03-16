use super::*;

/// A [`SimulationService`] that always returns an error.
///
/// This is useful for improving error messages if `fnsd` is not running with expensive RPCs
/// enabled.
pub struct SimulationsDisabled;

#[tonic::async_trait]
impl SimulationService for SimulationsDisabled {
    async fn simulate_trade(
        &self,
        _: tonic::Request<SimulateTradeRequest>,
    ) -> Result<tonic::Response<SimulateTradeResponse>, Status> {
        Err(Status::unimplemented(
            "SimulationService::simulate_trade() is not enabled on this node.\
             Run fnsd with `--enable-expensive-rpc` to use this RPC.",
        ))
    }
}
