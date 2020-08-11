use diesel::connection::{AnsiTransactionManager, SimpleConnection};
use diesel::deserialize::QueryableByName;
use diesel::pg::Pg;
use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
use diesel::r2d2::CustomizeConnection;
use diesel::sql_types::HasSqlType;
use diesel::{Connection, ConnectionResult, PgConnection, QueryResult, Queryable};
use rocket::Rocket;
use rocket_contrib::databases::{diesel, DatabaseConfig, Poolable};

embed_migrations!();

#[cfg(not(test))]
#[database("postgres")]
pub struct Database(diesel::PgConnection);

#[derive(Debug)]
struct TestTransaction;

impl CustomizeConnection<TestPgConnection, diesel::r2d2::Error> for TestTransaction {
    fn on_acquire(&self, conn: &mut TestPgConnection) -> Result<(), diesel::r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

/// A connection useful for testing. All transactions are rolled back and are never committed to the
/// database. This should be used only for testing and not for any other purpose.
pub struct TestPgConnection(diesel::PgConnection);

impl SimpleConnection for TestPgConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.0.batch_execute(query)
    }
}

impl Connection for TestPgConnection {
    type Backend = Pg;
    type TransactionManager = AnsiTransactionManager;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        Ok(Self(PgConnection::establish(database_url)?))
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        self.0.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Pg> + QueryId,
        Pg: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Pg>,
    {
        self.0.query_by_index(source)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Pg> + QueryId,
        U: QueryableByName<Pg>,
    {
        self.0.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Pg> + QueryId,
    {
        self.0.execute_returning_count(source)
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        &self.0.transaction_manager()
    }
}

impl Poolable for TestPgConnection {
    type Manager = diesel::r2d2::ConnectionManager<TestPgConnection>;
    type Error = rocket_contrib::databases::r2d2::Error;

    fn pool(config: DatabaseConfig<'_>) -> Result<diesel::r2d2::Pool<Self::Manager>, Self::Error> {
        let manager = diesel::r2d2::ConnectionManager::new(config.url);
        diesel::r2d2::Pool::builder()
            .connection_customizer(Box::new(TestTransaction))
            .max_size(config.pool_size)
            .build(manager)
    }
}

#[cfg(test)]
#[database("postgres")]
pub struct Database(TestPgConnection);

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
