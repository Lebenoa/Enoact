use axum::{Json, http::StatusCode, response::IntoResponse};
use discord_rich_presence::activity::Activity;
use serde::Deserialize;

use crate::sessions;

#[derive(Deserialize)]
pub struct SetPresenceRequest<'a> {
    activity: Activity<'a>,
    app_id: Option<String>,
}

pub async fn set_presence(
    Json(request_data): Json<SetPresenceRequest<'static>>,
) -> impl IntoResponse {
    if let Some(app_id) = request_data.app_id {
        match sessions::set_presence(&app_id, request_data.activity) {
            Ok(()) => (StatusCode::OK, app_id).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    } else {
        match sessions::new_session(request_data.activity) {
            Ok(client_id) => (StatusCode::OK, client_id).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}
