# standardnotes-rs

A Rust implementation of the [Standard Notes Sync specification](https://docs.standardnotes.org/specification/sync/).

## Technologies
* Rust (nightly, until the Rocket.rs 0.5.0 release)
* Rocket.rs
* Diesel.rs
* PostgreSQL
* Docker (not required)

## Getting Started

## .env

1. SN_SECRET: Secret used for creating JWTs

# POSTGRES DOCKER IMAGE ENV VARS (https://hub.docker.com/_/postgres/)
1. POSTGRES_DB: ?
2. POSTGRES_USER: ?
3. POSTGRES_PASSWORD: ?

1. DB_CONNECTION: ?
2. DB_HOST: ?
3. DB_PORT: ?
4. DB_USER: ?
5. DB_DATABASE: ?
6. DB_PASSWORD: ?

1. ROCKET_DATABASES: Database connection string to configure Rocket.rs 


### Running with a Local Postgres DB
1. Clone the project
2. Create a `.env` file (see .env.sample for an example)
3. Update the `.env` file to use your local Postgres connection details
4. Run the command `cargo run`
5. Start developing!

### Running with Docker
1. Clone the project
2. Create a `.env` file (see .env.sample for an example)
3. Run the command `docker compose up`
4. Start developing!

## Tests
There are currently no tests, but there will be!
