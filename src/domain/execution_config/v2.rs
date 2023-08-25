use crate::domain::addresses::{BLSPubkey, ExecutionAddress};

use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ExecutionConfig {
    pub version: u8,
    pub fee_recipient: ExecutionAddress,
    pub gas_limit: u64,
    pub grace: i64, // TODO: Duration
    pub min_value: Decimal,
    pub relays: HashMap<Url, BaseRelayConfig>,
    pub proposers: Vec<ProposerConfig>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BaseRelayConfig {
    pub public_key: BLSPubkey,
    pub fee_recipient: ExecutionAddress,
    pub gas_limit: u64,
    pub grace: i64, //TODO: Duration
    pub min_value: Decimal,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ProposerConfig {
    pub validator: BLSPubkey,
    pub account: String, //TODO: regexp
    pub fee_recipient: ExecutionAddress,
    pub gas_limit: u64,
    pub grace: i64,
    pub min_value: Decimal,
    pub reset_relays: bool,
    pub relays: HashMap<Url, ProposerRelayConfig>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct ProposerRelayConfig {
    pub disabled: bool,
    pub public_key: BLSPubkey,
    pub fee_recipient: ExecutionAddress,
    pub gas_limit: u64,
    pub grace: i64,
    pub min_value: Decimal,
}
