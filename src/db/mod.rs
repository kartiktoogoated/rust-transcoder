use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub mod models;
pub mod schema;
