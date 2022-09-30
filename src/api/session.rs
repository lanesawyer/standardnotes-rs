use crate::db::Database;
use crate::models::ApiResponse;
use crate::models::{RefreshResponse, RefreshSession, SessionsResponse};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[delete("/session", data = "<_uuid>")]
pub fn delete_session(_conn: Database, _uuid: String) -> ApiResponse<Status> {
    // TODO: Delete session
    Ok(Status::NoContent)
}

#[delete("/sessions")]
pub fn delete_sessions(_conn: Database) -> ApiResponse<Status> {
    // TODO: Delete all session for user except the current one
    Ok(Status::NoContent)
}

#[get("/sessions")]
pub fn get_sessions(_conn: Database) -> ApiResponse<Json<SessionsResponse>> {
    // TODO: Return list of sessions
    Ok(Json(SessionsResponse { sessions: vec![] }))
}

#[post("/session/token/refresh", data = "<_refresh_token>")]
pub fn refresh_session(
    _conn: Database,
    _refresh_token: String,
) -> ApiResponse<Json<RefreshResponse>> {
    // TODO: Refresh session
    Ok(Json(RefreshResponse {
        token: "blah".into(), // TODO: generate token
        session: RefreshSession {
            refresh_expiration: 5184000,  // 60 days
            refresh_token: "blah".into(), // TODO: refresh token
        },
    }))
}
