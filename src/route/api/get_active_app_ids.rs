use axum::{Json, response::IntoResponse};
use discord_rich_presence::activity::Activity;

use crate::sessions::HTTP_PRESENCE;

pub async fn active_app_ids() -> impl IntoResponse {
    let map_output: Vec<(String, Activity<'_>)> = HTTP_PRESENCE
        .iter()
        .map(|item| (item.key().to_string(), item.value().activity.clone()))
        .collect();

    Json(map_output)
}
