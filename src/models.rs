use crate::diesel::RunQueryDsl;
use crate::jwt::decode_jwt;
use crate::schema::{items, users};
// use chrono::{DateTime, Utc};
use diesel::pg::Pg;
use request::FromRequest;
use response::Responder;
use rocket::{
    http::{ContentType, Status},
    request, response, Outcome, Request, Response,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

pub type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
/// An error with the API.
pub struct ApiError {
    pub errors: Vec<String>,
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            // TODO: Use errors from self
            .sized_body(Cursor::new(format!(
                "{{\"errors\":[\"{}\"]}}",
                self.errors.join(", ")
            )))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub password: String,
    pub pw_cost: i64,
    pub pw_nonce: String,
    pub version: String,
}

impl User {
    pub fn create<C>(&self, conn: &C) -> bool
    where
        C: diesel::Connection<Backend = Pg>,
    {
        diesel::insert_into(users::table)
            .values(self)
            .get_result::<User>(conn)
            .expect("Error creating new user");

        true
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
    pub email: String,
    pub password: String,
    pub current_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub pw_cost: i64,
    pub pw_nonce: String,
    pub version: String,
}

impl From<&User> for Params {
    fn from(user: &User) -> Self {
        Params {
            pw_cost: user.pw_cost,
            pw_nonce: user.pw_nonce.clone(),
            version: user.version.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Sync {
    pub items: Vec<Item>,
    sync_token: String,
    limit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "items"]
pub struct Item {
    uuid: String,
    content: String,
    content_type: String,
    enc_item_key: String,
    deleted: bool,
    created_at: String, // DateTime<Utc>,
    updated_at: String, // DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SyncResponse {
    pub retrieved_items: Option<Vec<Item>>,
    pub saved_items: Option<Vec<Item>>,
    pub unsaved: Option<Vec<Item>>,
    pub sync_token: Option<String>,
}

#[derive(Debug)]
pub struct AuthUser {
    email: String,
    // TODO: Probably need more info for the signed in user
}

#[derive(Debug)]
pub struct ApiKeyError(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(header) = request.headers().get_one("Authorization") {
            if !header.starts_with("Bearer ") {
                return Outcome::Failure((
                    Status::Unauthorized,
                    ApiKeyError(String::from("Authorization header malformed")),
                ));
            }

            match decode_jwt(&header[7..]) {
                Ok(token) => {
                    return Outcome::Success(AuthUser {
                        email: token.claims.sub,
                    });
                }
                Err(e) => println!("{}", e),
            }
        } else {
            return Outcome::Failure((
                Status::Unauthorized,
                ApiKeyError(String::from("Authorization header missing")),
            ));
        }

        Outcome::Failure((
            Status::Unauthorized,
            ApiKeyError(String::from("Unable to authenticate")),
        ))
    }
}
