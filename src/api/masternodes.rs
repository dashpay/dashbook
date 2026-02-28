use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::masternode::{MasternodeDetail, MasternodeSummary};
use crate::AppError;
use crate::AppState;

#[derive(Deserialize)]
pub struct MasternodeListParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    #[serde(rename = "type")]
    pub mn_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct MasternodeListResponse {
    pub masternodes: Vec<MasternodeSummary>,
    pub total: usize,
    pub page: u32,
    pub pages: u32,
}

pub async fn list_masternodes(
    State(state): State<AppState>,
    Query(params): Query<MasternodeListParams>,
) -> Result<Json<MasternodeListResponse>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(50).min(200);

    let list = state.rpc.get_masternode_list().await?;

    let mut masternodes: Vec<MasternodeSummary> = list
        .values()
        .map(MasternodeSummary::from_list_entry)
        .collect();

    // Filter by type
    if let Some(ref mn_type) = params.mn_type {
        if mn_type != "all" {
            masternodes.retain(|mn| mn.mn_type.to_lowercase() == mn_type.to_lowercase());
        }
    }

    // Filter by status
    if let Some(ref status) = params.status {
        if status != "all" {
            masternodes.retain(|mn| mn.status.to_lowercase() == status.to_lowercase());
        }
    }

    // Sort: enabled first, then by PoSe penalty
    masternodes.sort_by(|a, b| {
        let a_enabled = a.status == "ENABLED";
        let b_enabled = b.status == "ENABLED";
        b_enabled
            .cmp(&a_enabled)
            .then(a.pose_penalty.cmp(&b.pose_penalty))
    });

    let total = masternodes.len();
    let pages = ((total as f64) / limit as f64).ceil() as u32;

    let start = ((page - 1) * limit) as usize;
    let paginated: Vec<MasternodeSummary> = masternodes
        .into_iter()
        .skip(start)
        .take(limit as usize)
        .collect();

    Ok(Json(MasternodeListResponse {
        masternodes: paginated,
        total,
        page,
        pages,
    }))
}

pub async fn get_masternode(
    State(state): State<AppState>,
    Path(protxhash): Path<String>,
) -> Result<Json<MasternodeDetail>, AppError> {
    let protx = state.rpc.get_protx_info(&protxhash).await?;
    Ok(Json(MasternodeDetail::from_protx(&protx)))
}
