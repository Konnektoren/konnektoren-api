use axum::extract::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[cfg(feature = "ton")]
use crate::services::v1::claim::claim_tokens_service;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRequest {
    #[schema(example = 123456)]
    pub id: i64,
    #[schema(example = "example_user")]
    pub user: String,
    #[serde(rename = "type")]
    #[schema(example = "claim")]
    pub request_type: String,
    #[schema(example = "0QB-_k5Rule-nKr6HWPIlkDyHb1xhDdbI77q7uwAFqmUmKjP")]
    pub address: String,
    #[schema(example = 1)]
    pub amount: u32,
}

#[cfg(feature = "ton")]
#[utoipa::path(
    post,
    operation_id = "claim_v1",
    tag = "claim_v1",
    path = "/claim",
    context_path = "/api/v1",
    request_body = ClaimRequest,
    responses(
        (status = 200, description = "Token claimed successfully"),
        (status = 400, description = "Invalid request data"),
    )
)]
pub async fn claim_tokens(
    Json(payload): Json<ClaimRequest>,
) -> Result<Json<&'static str>, (axum::http::StatusCode, String)> {
    claim_tokens_service(payload).await
}
