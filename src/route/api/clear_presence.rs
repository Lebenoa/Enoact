use axum::{http::StatusCode, response::IntoResponse};

use crate::sessions;

pub async fn clear_presence(client_id: String) -> impl IntoResponse {
    if let Err(e) = sessions::clear_presence(&client_id) {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }
    (StatusCode::OK).into_response()
}
