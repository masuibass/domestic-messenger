use super::AppState;
use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use service::member;

pub async fn list_members(state: State<AppState>) -> impl IntoResponse {
    let members = member::list(&state.conn)
        .await
        .expect("failed to list members");

    Json(members)
}

pub async fn create_member(
    state: State<AppState>,
    Json(payload): Json<CreateMember>,
) -> impl IntoResponse {
    let id = member::create(&state.conn, &payload.name)
        .await
        .expect("failed to create member");

    Json(id)
}

#[derive(Deserialize)]
pub struct CreateMember {
    name: String,
}
