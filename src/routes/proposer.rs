use crate::domain::addresses::BLSPubkey;
use crate::domain::execution_config::v1::ProposerConfig;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::get,
    Router,
};

pub fn get_router() -> Router {
    Router::new().route("/proposer/:pubkey", get(get_proposer))
}

#[tracing::instrument]
async fn get_proposer(Path(pubkey): Path<BLSPubkey>) -> (StatusCode, Json<ProposerConfig>) {
    let proposer = Default::default();
    (StatusCode::OK, Json(proposer))
}
