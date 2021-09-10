use crate::db::Database;
use crate::models::{AuthResponse, ChangePassword, KeyParams, Session, UserResponse};
use crate::{
    jwt::build_jwt,
    models::{ApiResponse, AuthUser, CreateUser, ParamsResponse, SignIn, User},
};
use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

use super::build_api_error;

#[post("/", data = "<create_user>")]
pub fn create_user(
    conn: Database,
    create_user: Json<CreateUser>,
) -> ApiResponse<Json<AuthResponse>> {
    // TODO: Better conversion
    // TODO: Fix User table
    let user = User {
        email: create_user.email.clone(),
        password: create_user.password.clone(),
        pw_cost: 100,
        pw_nonce: create_user.pw_nonce.clone(),
        version: create_user.version.clone(),
    };

    if user.create(&*conn) {
        let _token = match build_jwt(&user.email) {
            Ok(token) => token,
            Err(_err) => return Err(build_api_error("Error building JWT")),
        };

        Ok(Json(AuthResponse {
            session: Session::default(), // TODO: Session
            key_params: KeyParams {
                created: create_user.created.clone(),
                identifier: create_user.identifier.clone(),
                origination: create_user.origination.clone(),
                pw_nonce: create_user.pw_nonce.clone(),
                version: create_user.version.clone(),
            },
            user: UserResponse {
                uuid: "1".into(), // TODO: Add uuid for user
                email: user.email,
            },
        }))
    } else {
        // TODO: Get these errors to be returned in the response
        Err(build_api_error("Unable to create user account"))
    }
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(conn: Database, sign_in: Json<SignIn>) -> ApiResponse<Json<AuthResponse>> {
    use crate::schema::users::dsl::{email, password, users};

    let result = users
        .filter(email.eq(&sign_in.email))
        .filter(password.eq(&sign_in.password))
        .limit(1)
        .load::<User>(&*conn)
        .unwrap();
    let user = result.first().unwrap();

    let _token = match build_jwt(&user.email) {
        Ok(token) => token,
        Err(_err) => return Err(build_api_error("Error building JWT")),
    };

    Ok(Json(AuthResponse {
        session: Session::default(),
        key_params: KeyParams {
            created: "created todo".into(),       // TOOD: get created
            identifier: "todo ideintifer".into(), // TODO: get user?
            origination: "todo wat".into(),       // TODO: get from user?
            pw_nonce: "blkahh".into(),            // TODO: get from user?
            version: "004".into(),                // TODO: Get version from ?User?
        },
        user: UserResponse {
            uuid: "1".into(), // TODO: Add uuid for user
            email: user.email.clone(),
        },
    }))
}

// TODO: Session stuff
#[post("/sign_out")]
pub fn sign_out(_conn: Database) -> ApiResponse<Status> {
    Ok(Status::NoContent)
}

#[get("/params?<_email>&<_api>")]
pub fn params(
    _user: AuthUser,
    conn: Database,
    _email: String,
    _api: String,
) -> ApiResponse<Json<ParamsResponse>> {
    use crate::schema::users::dsl::{email, users};

    let result = users
        .filter(email.eq(email))
        .limit(1)
        .load::<User>(&*conn)
        .unwrap();
    let user = result.first().unwrap();

    Ok(Json(ParamsResponse::from(user)))
}

// TODO: Set headers for OPTIONS response at some point
#[options("/params/<_params_email>")]
pub fn params_options(_params_email: String) -> ApiResponse<Status> {
    Ok(Status::NoContent)
}

#[post("/change_pw", data = "<change_pw>")]
pub fn change_pw(
    conn: Database,
    _user: AuthUser,
    change_pw: Json<ChangePassword>,
) -> ApiResponse<Json<AuthResponse>> {
    use crate::schema::users::dsl::{password, users};
    diesel::update(users.find(&change_pw.identifier))
        .filter(password.eq(&change_pw.current_password))
        .set(password.eq(&change_pw.new_password))
        .get_result::<User>(&*conn)
        .expect("Error updating password");

    Ok(Json(AuthResponse {
        session: Session::default(),
        key_params: KeyParams {
            created: "created todo".into(),       // TOOD: get created
            identifier: "todo ideintifer".into(), // TODO: get user?
            origination: "todo wat".into(),       // TODO: get from user?
            pw_nonce: "blkahh".into(),            // TODO: get from user?
            version: "004".into(),                // TODO: Get version from ?User?
        },
        user: UserResponse {
            uuid: "1".into(),      // TODO: Add uuid for user
            email: "email".into(), // TODO: get user
        },
    }))
}
