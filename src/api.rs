use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::jwt::{build_jwt, Token};

#[post("/", data = "<user>")]
pub fn auth(user: Json<User>) -> Json<Token> {
    // TODO: Create user, handle errors

    Json(Token {
        token: build_jwt(user.email.to_owned()),
    })
}

#[post("/change_pw", data = "<change_pw>")]
pub fn change_pw(change_pw: Json<ChangePassword>) -> Status {
    // TODO: Change password, handle errors

    Status::NoContent
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(sign_in: Json<SignIn>) -> Json<Token> {
    // TODO: Check user info, handle errors

    Json(Token {
        token: build_jwt(sign_in.email.to_owned()),
    })
}

#[get("/params/<email>")]
pub fn params(email: String) -> Json<Token> {
    // TODO: Retrieve params, handle errors

    Json(Token {
        token: build_jwt(email),
    })
}

#[post("/sync", data = "<sync>")]
pub fn sync(sync: Json<Sync>) -> Json<Token> {
    // TODO: Sync the data, handle errors

    Json(Token {
        token: build_jwt("what".to_owned()),
    })
}

#[derive(Deserialize)]
pub struct User {
    email: String,
    password: String,
    pw_cost: String,
    pw_nonce: String,
    version: String,
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

#[derive(Serialize, Deserialize)]
struct Item {
    uuid: String,
    content: String,
    content_type: String,
    enc_item_key: String,
    deleted: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}