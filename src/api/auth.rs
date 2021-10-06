use crate::db::Database;
use crate::models::{ApiResponse, AuthUser, CreateUser, ParamsResponse, SignIn, User};
use crate::models::{AuthResponse, ChangePassword, NewUser};
use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<create_user>")]
pub fn create_user(
    conn: Database,
    create_user: Json<CreateUser>,
) -> ApiResponse<Json<AuthResponse>> {
    let new_user: NewUser = create_user.into();
    let created_user = new_user.create(&*conn);

    Ok(Json(AuthResponse::from(created_user)))
    // TODO: Handle error
    // } else {
    //     // TODO: Get these errors to be returned in the response
    //     Err(build_api_error("Unable to create user account"))
    // }
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(conn: Database, sign_in: Json<SignIn>) -> ApiResponse<Json<AuthResponse>> {
    use crate::schema::users::dsl::{email, password, users};

    let result = users
        .filter(email.eq(&sign_in.email))
        .filter(password.eq(&sign_in.password))
        .limit(1)
        .load::<User>(&*conn)
        .expect("Should've found user");

    let user = result.first().unwrap();

    Ok(Json(AuthResponse::from(user)))
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
    let user = diesel::update(users.find(&change_pw.identifier))
        .filter(password.eq(&change_pw.current_password))
        .set(password.eq(&change_pw.new_password))
        .get_result::<User>(&*conn)
        .expect("Error updating password");

    Ok(Json(AuthResponse::from(user)))
}
