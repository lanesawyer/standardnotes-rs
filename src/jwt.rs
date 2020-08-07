use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;

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

pub fn build_jwt(subject: String) -> String {
    let my_claims = Claims {
        sub: subject,
        iss: "StandardNotes".to_owned(),
        exp: 10000000000,
    };
    let token = match encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    token
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation {
            iss: Some("StandardNotes".to_owned()),
            ..Default::default()
        },
    )

    // let token_data = match  {
    //     Ok(c) => c,
    //     Err(err) => match *err.kind() {
    //         ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
    //         ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
    //         _ => panic!("Some other errors"),
    //     },
    // }
}

fn get_secret() -> String {
    env::var("SN_SECRET").expect("SN_SECRET environment variable not provided")
}
