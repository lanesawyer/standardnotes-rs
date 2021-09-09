# standardnotes-rs

A Rust implementation of the [Standard Notes Sync specification](https://docs.standardnotes.org/specification/sync/).

## Technologies
* Rust (nightly, until the Rocket.rs 0.5.0 release)
* Rocket.rs
* Diesel.rs
* PostgreSQL

## Getting Started
1. Clone the project
2. Create a `.env` file (see .env.sample for an example)
3. Create the Standard Notes database using `diesel migration run`
4. Update Rocket.toml to use your local database's username and password
5. Run the command `cargo run`
6. Start developing!

## Tests
There are currently no tests, but there will be!
