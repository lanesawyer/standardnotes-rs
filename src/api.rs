use crate::db::Database;
use crate::models::{AuthResponse, ChangePassword, KeyParams, Session, UserResponse};
use crate::{
    jwt::build_jwt,
    models::{
        ApiError, ApiResponse, AuthUser, CreateUser, Item, ParamsResponse, SignIn, Sync,
        SyncResponse, User,
    },
};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::Request;
use rocket_contrib::json::Json;

#[post("/", data = "<create_user>")]
pub fn create_user(
    conn: Database,
    create_user: Json<CreateUser>,
) -> ApiResponse<Json<AuthResponse>> {
    // TODO: Better conversion
    let user = User {
        email: create_user.email.clone(),
        password: create_user.password.clone(),
        pw_cost: 100,
        pw_nonce: create_user.pw_nonce.clone(),
        version: create_user.version.clone(),
    };
    if user.create(&*conn) {
        let token = match build_jwt(&user.email) {
            Ok(token) => token,
            Err(_err) => return Err(build_api_error("Error building JWT")),
        };

        // TODO: Actual data
        Ok(Json(AuthResponse {
            session: Session::default(),
            key_params: KeyParams::default(),
            user: UserResponse::default(),
        }))
    } else {
        // TODO: Get these errors to be returned in the response
        Err(build_api_error("Unable to create user account"))
    }
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
        key_params: KeyParams::default(),
        user: UserResponse::default(),
    }))
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(conn: Database, sign_in: Json<SignIn>) -> ApiResponse<Json<AuthResponse>> {
    // use crate::schema::users::dsl::{email, password, users};

    // let result = users
    //     .filter(email.eq(&sign_in.email))
    //     .filter(password.eq(&sign_in.password))
    //     .limit(1)
    //     .load::<User>(&*conn)
    //     .unwrap();
    // let user = result.first().unwrap();

    // let token = match build_jwt(&user.email) {
    //     Ok(token) => token,
    //     Err(_err) => return Err(build_api_error("Error building JWT")),
    // };

    Ok(Json(AuthResponse {
        session: Session::default(),
        key_params: KeyParams::default(),
        user: UserResponse::default(),
    }))
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

#[post("/sync", data = "<sync>")]
#[allow(unused_variables)]
pub fn sync(_user: AuthUser, conn: Database, sync: Json<Sync>) -> ApiResponse<Json<SyncResponse>> {
    use crate::schema::items::dsl::items;

    let sync = sync.into_inner();
    match diesel::insert_into(items)
        .values(&sync.items)
        .get_result::<Item>(&*conn)
    {
        Ok(item) => {
            let hi = "wat";

            // these items are new or have been modified since last sync and should be merged or created locally.

            Ok(Json(SyncResponse {
                saved_items: Some(sync.items),
                retrieved_items: None,
                unsaved: None,
                sync_token: None,
            }))
        }
        Err(_) => Err(build_api_error("Error syncing item")),
    }
}

#[catch(400)]
pub fn bad_request(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Bad request"))
}

#[catch(401)]
pub fn unauthorized(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Unauthorized"))
}

#[catch(404)]
pub fn not_found(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Not found"))
}

#[catch(500)]
pub fn server_error(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(build_api_error("Server error"))
}

fn build_api_error(error_message: &str) -> ApiError {
    ApiError {
        errors: vec![error_message.into()],
    }
}
