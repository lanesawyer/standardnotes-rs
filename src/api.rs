use crate::db::Database;
use crate::{
    jwt::{build_jwt, Token},
    models::{
        ApiError, ApiResponse, AuthUser, ChangePassword, Item, Params, SignIn, Sync, SyncResponse,
        User,
    },
};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::Request;
use rocket_contrib::json::Json;

#[post("/", data = "<user>")]
pub fn auth(user: Json<User>, conn: Database) -> ApiResponse<Json<Token>> {
    if user.create(&*conn) {
        let token = match build_jwt(&user.email) {
            Ok(token) => token,
            Err(_err) => return Err(build_api_error("Error building JWT")),
        };

        Ok(Json(Token { token }))
    } else {
        // TODO: Get these errors to be returned in the response
        Err(build_api_error("Unable to create user account"))
    }
}

#[post("/change_pw", data = "<change_pw>")]
pub fn change_pw(
    _user: AuthUser,
    change_pw: Json<ChangePassword>,
    conn: Database,
) -> ApiResponse<Status> {
    use crate::schema::users::dsl::{password, users};
    diesel::update(users.find(&change_pw.email))
        .filter(password.eq(&change_pw.current_password))
        .set(password.eq(&change_pw.password))
        .get_result::<User>(&*conn)
        .expect("Error updating password");

    Ok(Status::NoContent)
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(sign_in: Json<SignIn>, conn: Database) -> ApiResponse<Json<Token>> {
    use crate::schema::users::dsl::{email, password, users};

    let result = users
        .filter(email.eq(&sign_in.email))
        .filter(password.eq(&sign_in.password))
        .limit(1)
        .load::<User>(&*conn)
        .unwrap();
    let user = result.first().unwrap();

    let token = match build_jwt(&user.email) {
        Ok(token) => token,
        Err(_err) => return Err(build_api_error("Error building JWT")),
    };

    Ok(Json(Token { token }))
}

#[get("/params?<_email>&<_api>")]
pub fn params(
    _user: AuthUser,
    conn: Database,
    _email: String,
    _api: String,
) -> ApiResponse<Json<Params>> {
    use crate::schema::users::dsl::{email, users};

    let result = users
        .filter(email.eq(email))
        .limit(1)
        .load::<User>(&*conn)
        .unwrap();
    let user = result.first().unwrap();

    Ok(Json(Params::from(user)))
}

// TODO: Set headers for OPTIONS response at some point
#[options("/params/<_params_email>")]
pub fn params_options(_params_email: String) -> ApiResponse<Status> {
    Ok(Status::NoContent)
}

#[post("/sync", data = "<sync>")]
#[allow(unused_variables)]
pub fn sync(_user: AuthUser, sync: Json<Sync>, conn: Database) -> ApiResponse<Json<SyncResponse>> {
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
