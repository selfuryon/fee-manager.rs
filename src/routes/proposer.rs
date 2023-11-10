use crate::domain::addresses::BLSPubkey;
use crate::domain::execution_config::v1::ProposerConfig;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use sqlx::postgres::PgPool;

pub fn router(pool: PgPool) -> Router<()> {
    Router::new()
        .route("/proposer/:pubkey", get(get_proposer).put(put_proposer))
        .with_state(pool)
}

#[tracing::instrument]
async fn get_proposer(
    Path(pubkey): Path<BLSPubkey>,
    state: State<PgPool>,
) -> (StatusCode, Json<ProposerConfig>) {
    let proposer = Default::default();
    (StatusCode::OK, Json(proposer))
}

#[tracing::instrument]
async fn put_proposer(
    Path(pubkey): Path<BLSPubkey>,
    Json(payload): Json<ProposerConfig>,
) -> StatusCode {
    StatusCode::OK
}
