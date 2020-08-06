#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod db;
mod jwt;
mod schema;

fn main() {
    rocket::ignite()
        .mount(
            "/auth",
            routes![api::auth, api::change_pw, api::sign_in, api::params],
        )
        .mount("/items", routes![api::sync])
        .launch();
}
