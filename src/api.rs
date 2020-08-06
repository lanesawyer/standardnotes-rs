use crate::{db::establish_connection, jwt::{build_jwt, decode_jwt, Token}};
use chrono::{DateTime, Utc};
use request::FromRequest;
use response::Responder;
use rocket::{
    http::{ContentType, Status},
    request, response, Outcome, Request, Response,
};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use jsonwebtoken::errors::ErrorKind;
use diesel::prelude::*;
use super::schema::users;


#[post("/", data = "<user>")]
pub fn auth(user: Json<User>) -> ApiResponse<Json<Token>> {
    let connection = establish_connection();
    if user.save(&connection) {
        Ok(Json(Token {
            token: build_jwt(user.email.to_owned()),
        }))
    } else {
        Err(ApiError {
            errors: vec!["Unable to create user account".to_owned()],
        })
    }
}

#[post("/change_pw", data = "<change_pw>")]
pub fn change_pw(user: AuthUser, change_pw: Json<ChangePassword>) -> ApiResponse<Status> {
    // TODO: Change password, handle errors

    Ok(Status::NoContent)
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(user: AuthUser, sign_in: Json<SignIn>) -> ApiResponse<Json<Token>> {
    // TODO: Check user info, handle errors

    Ok(Json(Token {
        token: build_jwt(sign_in.email.to_owned()),
    }))
}

#[get("/params/<email>")]
pub fn params(user: AuthUser, email: String) -> ApiResponse<Json<Token>> {
    // TODO: Retrieve params, handle errors

    Ok(Json(Token {
        token: build_jwt(email),
    }))
}

#[post("/sync", data = "<sync>")]
pub fn sync(user: AuthUser, sync: Json<Sync>) -> ApiResponse<Json<Token>> {
    // TODO: Sync the data, handle errors

    Ok(Json(Token {
        token: build_jwt("what".to_owned()),
    }))
}

type ApiResponse<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    errors: Vec<String>,
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
#[table_name="users"]
pub struct User {
    email: String,
    password: String,
    pw_cost: String,
    pw_nonce: String,
    version: String,
}

impl User {
    fn save(&self, conn: &PgConnection) -> bool {
        use crate::schema::users;
    
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
    email: String,
    password: String,
    current_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Params {
    pw_cost: String,
    pw_nonce: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sync {
    items: Vec<Item>,
    sync_token: String,
    limit: String,
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
    email: String
}

#[derive(Debug)]
pub struct ApiKeyError(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        if let Some(header) = request.headers().get_one("Authorization") {
            if !header.starts_with("Bearer ") {
                return Outcome::Failure((Status::Unauthorized, ApiKeyError("Authorization header malformed".to_owned())));
            }

            if let Ok(claim) = decode_jwt(header[7..].to_owned()) {
                return Outcome::Success(AuthUser {
                    email: claim.claims.sub
                });
            }
        } else {
            return Outcome::Failure((Status::Unauthorized, ApiKeyError("Authorization header missing".to_owned())))
        }

        Outcome::Failure((Status::Unauthorized, ApiKeyError("Unable to authenticate".to_owned())))
    }
}
