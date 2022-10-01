// use diesel::connection::{AnsiTransactionManager, SimpleConnection};
// use diesel::deserialize::QueryableByName;
// use diesel::pg::Pg;
// use diesel::query_builder::{AsQuery, QueryFragment, QueryId};
// use diesel::r2d2::CustomizeConnection;
// use diesel::sql_types::HasSqlType;
// use diesel::{Connection, ConnectionResult, PgConnection, QueryResult, Queryable};
use rocket::{Rocket, Build};
use rocket_sync_db_pools::{database, diesel};

embed_migrations!();

// #[cfg(not(test))]
#[database("postgres")]
pub struct Database(diesel::PgConnection);

// #[derive(Debug)]
// struct TestTransaction;

// impl CustomizeConnection<TestPgConnection, diesel::r2d2::Error> for TestTransaction {
//     fn on_acquire(&self, conn: &mut TestPgConnection) -> Result<(), diesel::r2d2::Error> {
//         conn.begin_test_transaction().unwrap();
//         Ok(())
//     }
// }

// A connection useful for testing. All transactions are rolled back and are never committed to the
// database. This should be used only for testing and not for any other purpose.
// pub struct TestPgConnection(diesel::PgConnection);

// impl SimpleConnection for TestPgConnection {
//     fn batch_execute(&self, query: &str) -> QueryResult<()> {
//         self.0.batch_execute(query)
//     }
// }

// impl Connection for TestPgConnection {
//     type Backend = Pg;
//     type TransactionManager = AnsiTransactionManager;

//     fn establish(database_url: &str) -> ConnectionResult<Self> {
//         Ok(Self(PgConnection::establish(database_url)?))
//     }

//     fn execute(&self, query: &str) -> QueryResult<usize> {
//         self.0.execute(query)
//     }

//     fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
//     where
//         T: AsQuery,
//         T::Query: QueryFragment<Pg> + QueryId,
//         Pg: HasSqlType<T::SqlType>,
//         U: Queryable<T::SqlType, Pg>,
//     {
//         self.0.query_by_index(source)
//     }

//     fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
//     where
//         T: QueryFragment<Pg> + QueryId,
//         U: QueryableByName<Pg>,
//     {
//         self.0.query_by_name(source)
//     }

//     fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
//     where
//         T: QueryFragment<Pg> + QueryId,
//     {
//         self.0.execute_returning_count(source)
//     }

//     fn transaction_manager(&self) -> &Self::TransactionManager {
//         self.0.transaction_manager()
//     }
// }

// impl Poolable for TestPgConnection {
//     type Manager = diesel::r2d2::ConnectionManager<TestPgConnection>;
//     type Error = rocket_sync_db_pools::r2d2::Error;

//     fn pool(db_name: &str, rocket: &Rocket<Build>) -> PoolResult<Self> {
//         let manager = diesel::r2d2::ConnectionManager::new(config.url);
//         let the_thing = diesel::r2d2::Pool::builder()
//             .connection_customizer(Box::new(TestTransaction))
//             .max_size(config.pool_size)
//             .build(manager);
//         the_thing
//     }
// }

// #[cfg(test)]
// #[database("postgres")]
// pub struct Database(TestPgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    let conn = Database::get_one(&rocket).await.expect("Couldn't create a database connection.");
    conn.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    rocket
}
