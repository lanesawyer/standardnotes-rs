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
            Err(_err) => panic!("deal with this"),
        };

        Ok(Json(Token { token }))
    } else {
        // TODO: Get these errors to be returned in the response
        Err(ApiError {
            errors: vec![String::from("Unable to create user account")],
        })
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
pub fn sign_in(sign_in: Json<SignIn>) -> ApiResponse<Json<Token>> {
    use crate::schema::users::dsl::{email, password, users};

    let connection = establish_connection();
    let result = users
        .filter(email.eq(&sign_in.email))
        .filter(password.eq(&sign_in.password))
        .limit(1)
        .load::<User>(&connection)
        .unwrap();
    let user = result.first().unwrap();

    let token = match build_jwt(&user.email) {
        Ok(token) => token,
        Err(_err) => panic!("deal with this"),
    };

    Ok(Json(Token { token }))
}

#[get("/params/<params_email>")]
pub fn params(_user: AuthUser, params_email: String, conn: Database) -> ApiResponse<Json<Params>> {
    use crate::schema::users::dsl::{email, users};

    let result = users
        .filter(email.eq(params_email))
        .limit(1)
        .load::<User>(&*conn)
        .unwrap();
    let user = result.first().unwrap();

    Ok(Json(Params::from(user)))
}

#[post("/sync", data = "<sync>")]
#[allow(unused_variables)]
pub fn sync(_user: AuthUser, sync: Json<Sync>, conn: Database) -> ApiResponse<Json<SyncResponse>> {
    // TODO: Sync the data, handle errors
    use crate::schema::items::dsl::*;

    let item = sync.0;
    let something = diesel::insert_into(items)
        .values(item.items)
        .get_result::<Item>(&*conn)
        .expect("Error creating new user");

    Ok(Json(SyncResponse {
        saved_items: Some(vec![something]),
        retrieved_items: None,
        unsaved: None,
        sync_token: None,
    }))
}

#[catch(400)]
pub fn bad_request(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(ApiError {
        errors: vec![String::from("Bad request")],
    })
}

#[catch(404)]
pub fn not_found(_req: &Request) -> ApiResponse<Json<SyncResponse>> {
    Err(ApiError {
        errors: vec![String::from("Not found")],
    })
}
