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

use dotenv::dotenv;
use rocket::fairing::AdHoc;

mod api;
mod db;
mod jwt;
mod models;
mod schema;

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(db::Database::fairing())
        .attach(AdHoc::on_attach("Database Migrations", db::run_migrations))
        .mount(
            "/auth",
            routes![api::auth, api::change_pw, api::sign_in, api::params],
        )
        .mount("/items", routes![api::sync])
        .register(catchers![
            api::bad_request,
            api::unauthorized,
            api::not_found,
            api::server_error
        ])
        .launch();
}
