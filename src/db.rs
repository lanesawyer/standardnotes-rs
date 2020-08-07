use rocket::Rocket;
use rocket_contrib::databases::diesel;

embed_migrations!();

#[database("postgres")]
pub struct Database(diesel::PgConnection);

pub fn run_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let conn = Database::get_one(&rocket).expect("Couldn't create a database connection.");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

