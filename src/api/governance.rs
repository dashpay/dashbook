use axum::extract::State;
use axum::Json;

use crate::models::governance::{GovernanceInfo, GovernanceOverview, Proposal};
use crate::AppError;
use crate::AppState;

pub async fn get_governance(
    State(state): State<AppState>,
) -> Result<Json<GovernanceOverview>, AppError> {
    let (info_res, objects_res) = tokio::join!(
        state.rpc.get_governance_info(),
        state.rpc.get_governance_objects(),
    );

    let info = info_res?;
    let objects = objects_res?;

    let mut proposals: Vec<Proposal> = objects
        .iter()
        .filter_map(|(hash, obj)| Proposal::from_rpc(hash, obj))
        .collect();

    // Sort by creation time, newest first
    proposals.sort_by(|a, b| b.creation_time.cmp(&a.creation_time));

    Ok(Json(GovernanceOverview {
        info: GovernanceInfo::from_rpc(&info),
        proposals,
    }))
}
