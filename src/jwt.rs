use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

const ISSUER: &str = "StandardNotesRustServer";
const _EXPIRATION_TIME: usize = 10000000000;

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

// TODO: Remove all this once confirmed it's not being used in new session work
pub fn _build_jwt(subject: &str) -> Result<String, Error> {
    let token = encode(
        &Header::default(),
        &Claims {
            sub: subject.to_string(),
            iss: String::from(ISSUER),
            exp: _EXPIRATION_TIME,
        },
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation {
            iss: Some(String::from(ISSUER)),
            ..Default::default()
        },
    )?;

    Ok(token)
}

fn get_secret() -> String {
    env::var("SN_SECRET").expect("SN_SECRET environment variable must be provided")
}

#[cfg(test)]
mod tests {
    #[test]
    fn jwt_encodes_and_decodes_correctly() {
        std::env::set_var("SN_SECRET", "test_secret");

        let token = super::_build_jwt("test@test.com").unwrap();
        let decoded_token = super::decode_jwt(&token).unwrap();

        assert_eq!(decoded_token.claims.sub, "test@test.com");
        assert_eq!(decoded_token.claims.iss, super::ISSUER);
        assert_eq!(decoded_token.claims.exp, super::_EXPIRATION_TIME);
    }

    #[test]
    fn get_secret_returns_env_var() {
        std::env::set_var("SN_SECRET", "test_secret");

        let secret = super::get_secret();

        assert_eq!(secret, "test_secret");
    }

    #[test]
    #[should_panic]
    fn get_secret_panics_without_env_var() {
        std::env::remove_var("SN_SECRET");
        super::get_secret();
    }
}
