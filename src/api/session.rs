use crate::db::Database;
use crate::{
    models::ApiResponse,
};
use rocket::http::Status;

#[delete("/session", data = "<uuid>")]
pub fn delete_session(conn: Database, uuid: String) -> ApiResponse<Status> {
    Ok(Status::NoContent)
}

#[delete("/sessions")]
pub fn delete_sessions(conn: Database) -> ApiResponse<Status> {
    Ok(Status::NoContent)
}

#[get("/sessions")]
pub fn get_sessions(conn: Database) -> ApiResponse<Status> {
    // TODO: Return list of sessions
    Ok(Status::Ok)
}

#[post("/session/token/refresh", data = "<refresh_token>")]
pub fn refresh_session(conn: Database, refresh_token: String) -> ApiResponse<Status> {
    // TODO: Refresh session
    Ok(Status::Ok)
}
