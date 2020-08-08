use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

const ISS: &str = "StandardNotesRustServer";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    iss: String,
    exp: usize,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub token: String,
}

pub fn build_jwt(subject: &String) -> Result<String, Error> {
    let token = encode(
        &Header::default(),
        &Claims {
            sub: subject.clone(),
            iss: String::from(ISS),
            exp: 10000000000,
        },
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation {
            iss: Some(String::from(ISS)),
            ..Default::default()
        },
    )?;

    Ok(claims)
}

fn get_secret() -> String {
    env::var("SN_SECRET").expect("SN_SECRET environment variable must be provided")
}
