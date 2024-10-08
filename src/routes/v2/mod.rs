use crate::storage::{ProfileRepository, Storage};
use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod claim;

pub fn create_router() -> Router<Arc<Mutex<dyn Storage>>> {
    let router = Router::new();

    #[cfg(feature = "ton")]
    let router = router.route("/claim", post(claim::claim_tokens));

    router
}
