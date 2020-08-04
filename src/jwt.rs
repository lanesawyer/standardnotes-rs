use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

pub fn build_jwt(subject: String) -> String {
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
