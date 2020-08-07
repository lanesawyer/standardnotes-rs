use crate::diesel::RunQueryDsl;
use crate::jwt::decode_jwt;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::PgConnection;
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
pub struct ApiError {
    pub errors: Vec<String>,
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            // TODO: Use errors from self
            .sized_body(Cursor::new(format!(
                "{{errors:[{}]}}",
                "replace with errors"
            )))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}

#[derive(Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    password: String,
    pub pw_cost: String,
    pub pw_nonce: String,
    pub version: String,
}

impl User {
    pub fn create(&self, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(self)
            .get_result::<User>(conn)
            .expect("Error saving new user");

        // use crate::schema::users::dsl::*;

        // let connection = establish_connection();
        // let results = users
        //     .limit(5)
        //     .load::<User>(&connection)
        //     .expect("Error loading users");

        // println!("Displaying {} users", results.len());
        // for user in results {
        //     println!("{}", user.email);
        // }
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
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub pw_cost: String,
    pub pw_nonce: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sync {
    items: Vec<Item>,
    sync_token: String,
    limit: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable)]
struct Item {
    uuid: String,
    content: String,
    content_type: String,
    enc_item_key: String,
    deleted: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct AuthUser {
    email: String,
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
                    ApiKeyError("Authorization header malformed".to_owned()),
                ));
            }

            match decode_jwt(header[7..].to_owned()) {
                Ok(claim) => {
                    return Outcome::Success(AuthUser {
                        email: claim.claims.sub,
                    });
                }
                Err(e) => println!("{}", e),
            }
        } else {
            return Outcome::Failure((
                Status::Unauthorized,
                ApiKeyError("Authorization header missing".to_owned()),
            ));
        }

        Outcome::Failure((
            Status::Unauthorized,
            ApiKeyError("Unable to authenticate".to_owned()),
        ))
    }
}
