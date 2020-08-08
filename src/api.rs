use crate::{
    db::establish_connection,
    jwt::{build_jwt, Token},
    models::{ApiError, ApiResponse, AuthUser, ChangePassword, Params, SignIn, Sync, User},
};
use diesel::prelude::*;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<user>")]
pub fn auth(user: Json<User>) -> ApiResponse<Json<Token>> {
    let connection = establish_connection();
    if user.create(&connection) {
        let token = match build_jwt(&user.email) {
            Ok(token) => token,
            Err(err) => panic!("deal with this"),
        };

        Ok(Json(Token { token }))
    } else {
        // TODO: Get those errors to be returned in the response
        Err(ApiError {
            errors: vec![String::from("Unable to create user account")],
        })
    }
}

#[post("/change_pw", data = "<change_pw>")]
pub fn change_pw(_user: AuthUser, change_pw: Json<ChangePassword>) -> ApiResponse<Status> {
    use crate::schema::users::dsl::{password, users};

    let connection = establish_connection();

    diesel::update(users.find(&change_pw.email))
        .filter(password.eq(&change_pw.current_password))
        .set(password.eq(&change_pw.password))
        .get_result::<User>(&connection)
        .expect("Error updating password");

    Ok(Status::NoContent)
}

#[post("/sign_in", data = "<sign_in>")]
pub fn sign_in(sign_in: Json<SignIn>) -> ApiResponse<Json<Token>> {
    // TODO: Check user info, handle errors
    let token = match build_jwt(&sign_in.email) {
        Ok(token) => token,
        Err(err) => panic!("deal with this"),
    };

    Ok(Json(Token { token }))
}

#[get("/params/<params_email>")]
pub fn params(_user: AuthUser, params_email: String) -> ApiResponse<Json<Params>> {
    use crate::schema::users::dsl::{email, users};

    let connection = establish_connection();
    let result = users
        .filter(email.eq(params_email))
        .limit(1)
        .load::<User>(&connection)
        .unwrap();
    let user = result.first().unwrap();

    Ok(Json(Params::from(user)))
}

#[post("/sync", data = "<sync>")]
pub fn sync(_user: AuthUser, sync: Json<Sync>) -> ApiResponse<Json<Token>> {
    // TODO: Sync the data, handle errors
    let token = match build_jwt(&String::from("what")) {
        Ok(token) => token,
        Err(err) => panic!("deal with this"),
    };

    Ok(Json(Token {
        token
    }))
}
