use crate::models::{ApiError, ApiResponse, SyncResponse};
use rocket::Request;
use rocket_contrib::json::Json;

pub mod auth;
pub mod session;
pub mod sync;

#[catch(400)]
pub fn bad_request(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Bad request"))
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Unauthorized"))
}

#[catch(404)]
pub fn not_found(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Not found"))
}

#[catch(500)]
pub fn server_error(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Server error"))
}

fn build_api_error(error_message: &str) -> ApiError {
    ApiError {
        errors: vec![error_message.into()],
    }
}
