mod execution;
mod proposer;
pub use execution::ExecutionAddress;
pub use proposer::ProposerAddress;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ExecutionConfig {
    pub proposer_config: HashMap<ProposerAddress, ProposerConfig>,
    pub default_config: ProposerConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ProposerConfig {
    pub fee_recipient: ExecutionAddress,
    pub gas_limit: i64,
    pub builder: BuilderConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BuilderConfig {
    pub enable: bool,
    pub grace: i64,
    pub relays: Vec<Url>,
}
