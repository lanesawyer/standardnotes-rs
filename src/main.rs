#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/auth", data = "<user>")]
fn auth(user: Json<User>) -> Json<Token> {
    Json(Token {
        token: build_jwt(user.email.to_owned()),
    })
}

#[post("/auth/change_pw", data = "<change_pw>")]
fn change_pw(change_pw: Json<ChangePassword>) -> Status {
    Status::NoContent
}

#[post("/auth/sign_in", data = "<sign_in>")]
fn sign_in(sign_in: Json<SignIn>) -> Json<Token> {
    Json(Token {
        token: build_jwt(sign_in.email.to_owned()),
    })
}

#[get("/auth/params/<email>")]
fn params(email: String) -> Json<Token> {
    Json(Token {
        token: build_jwt(email),
    })
}

#[post("/items/sync", data = "<sync>")]
fn sync(sync: Json<Sync>) -> Json<Token> {
    Json(Token {
        token: build_jwt("what".to_owned()),
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, auth, change_pw, sign_in, params, sync])
        .launch();
}

#[derive(Deserialize)]
struct User {
    email: String,
    password: String,
    pw_cost: String,
    pw_nonce: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
struct ChangePassword {
    email: String,
    password: String,
    current_password: String,
}

#[derive(Serialize, Deserialize)]
struct SignIn {
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
struct Sync {
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Serialize)]
struct Token {
    token: String,
}

fn build_jwt(subject: String) -> String {
    let key = env::var("SN_SECRET").expect("No secret provided");
    let my_claims = Claims {
        sub: subject,
        company: "SN".to_owned(),
        exp: 10000000000,
    };
    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    token
}

fn validate_jwt(subject: String, token: String) {
    let key = env::var("SN_SECRET").expect("No secret provided");
    let validation = Validation {
        sub: Some(subject),
        ..Validation::default()
    };
    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("Some other errors"),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}
