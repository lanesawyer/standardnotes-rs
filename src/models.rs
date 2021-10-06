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
use rocket_contrib::json::Json;
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

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub api: String,
    pub created: String,
    pub email: String,
    pub identifier: String,
    pub origination: String,
    pub password: String,
    pub pw_nonce: String,
    pub version: String,
}

impl From<Json<CreateUser>> for NewUser {
    fn from(create_user: Json<CreateUser>) -> Self {
        // TODO: Improve clone usage
        NewUser {
            api: create_user.api.clone(),
            created: create_user.created.clone(),
            email: create_user.email.clone(),
            identifier: create_user.identifier.clone(),
            origination: create_user.origination.clone(),
            password: create_user.password.clone(),
            pw_nonce: create_user.pw_nonce.clone(),
            version: create_user.version.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Queryable)]
pub struct User {
    pub uuid: String, // TODO: Guid type
    pub api: String,
    pub created: String,
    pub email: String,
    pub identifier: String,
    pub origination: String,
    pub password: String,
    pub pw_nonce: String,
    pub version: String,
}

impl NewUser {
    pub fn create<C>(&self, conn: &C) -> User
    where
        C: diesel::Connection<Backend = Pg>,
    {
        diesel::insert_into(users::table)
            .values(self)
            .get_result::<User>(conn)
            .expect("Error creating new user") // TODO: Return result
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub api: String,
    pub created: String,
    pub email: String,
    pub ephemeral: bool,
    pub identifier: String,
    pub origination: String,
    pub password: String,
    pub pw_nonce: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChangePassword {
    pub api: String,
    pub created: String,
    pub identifier: String,
    pub origination: String,
    pub current_password: String,
    pub new_password: String,
    pub pw_nonce: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    pub api: String,
    pub email: String,
    pub ephemeral: bool,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    pub api: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct ParamsResponse {
    pub identifier: String,
    pub pw_nonce: String,
    pub version: String,
}

impl From<&User> for ParamsResponse {
    fn from(user: &User) -> Self {
        ParamsResponse {
            identifier: user.identifier.clone(),
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

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub session: Session,
    pub key_params: KeyParams,
    pub user: UserResponse,
}

impl From<User> for AuthResponse {
    fn from(user: User) -> Self {
        AuthResponse {
            session: Session::default(),
            key_params: KeyParams {
                created: user.created.clone(),
                identifier: user.identifier.clone(),
                origination: user.origination.clone(),
                pw_nonce: user.pw_nonce.clone(),
                version: user.version.clone(),
            },
            user: UserResponse {
                uuid: user.uuid.clone(),
                email: user.email,
            },
        }
    }
}

// TODO: Have only one From
impl From<&User> for AuthResponse {
    fn from(user: &User) -> Self {
        AuthResponse {
            session: Session::default(),
            key_params: KeyParams {
                created: user.created.clone(),
                identifier: user.identifier.clone(),
                origination: user.origination.clone(),
                pw_nonce: user.pw_nonce.clone(),
                version: user.version.clone(),
            },
            user: UserResponse {
                uuid: user.uuid.clone(),
                email: user.email.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expiration: usize,
    pub refresh_expiration: usize,
}

impl Session {
    pub fn _new() -> Self {
        Session {
            access_token: "blah".into(),
            refresh_token: "blah".into(),
            access_expiration: 5184000, // 60 days
            refresh_expiration: 31557600, // 1 year
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct KeyParams {
    pub created: String,
    pub identifier: String,
    pub origination: String,
    pub pw_nonce: String,
    pub version: String,
}

#[derive(Debug, Serialize, Default)]
pub struct UserResponse {
    pub uuid: String,
    pub email: String,
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

#[derive(Debug, Serialize)]
pub struct FullSession {
    pub uuid: String,
    pub user_uuid: String,
    pub user_agent: String,
    pub api_version: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expiration: String, // TODO: datetime
    pub renew_expiration: String,  // TODO: datetime
    pub created_at: String,        // TODO: datetime
    pub updated_at: String,        // TODO: datetime
}

#[derive(Debug, Serialize)]
pub struct SessionsResponse {
    pub sessions: Vec<SessionResponse>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub uuid: String,
    pub user_agent: String,
    pub api_version: String,
    pub current: bool,
    pub created_at: String, // TODO: datetime
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Debug, Serialize)]
pub struct Error {
    tag: String,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub token: String,
    pub session: RefreshSession,
}

#[derive(Debug, Serialize)]
pub struct RefreshSession {
    pub refresh_expiration: usize, // TODO: datetime?
    pub refresh_token: String,
}
