#![deny(
    missing_docs,
    missing_debug_implementations,
    bare_trait_objects,
    anonymous_parameters,
    unused_imports
)]

//! A synchronisation server for Standard Notes, written in Rust.
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

#[cfg(test)]
mod tests;

use dotenv::dotenv;
use rocket::fairing::AdHoc;

mod api;
mod db;
mod jwt;
mod models;
mod schema;

/// Makes a rocket that is ready for launch
pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(db::Database::fairing())
        .attach(AdHoc::on_attach("Database Migrations", db::run_migrations))
        .mount(
            "/auth",
            routes![
                api::auth::create_user,
                api::auth::sign_in,
                api::auth::sign_out,
                api::auth::params,
                api::auth::params_options,
                api::auth::change_pw
            ],
        )
        .mount(
            "/",
            routes![
                api::session::delete_session,
                api::session::delete_sessions,
                api::session::get_sessions,
                api::session::refresh_session,
            ],
        )
        .mount("/items", routes![api::sync::sync])
        .register(catchers![
            api::bad_request,
            api::unauthorized,
            api::not_found,
            api::server_error
        ])
}

fn main() {
    dotenv().ok();

    rocket().launch();
}
