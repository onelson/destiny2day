//! This module is largely just lightly modified versions of that you'd find in the
//! rocket guide around "managed state" https://rocket.rs/guide/state/#databases

use std::env;
use dotenv::dotenv;

use diesel::sqlite::SqliteConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

/// Initializes a database pool.
pub fn init_pool(pool_size: Option<u32>) -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = if let Some(size) = pool_size {
        r2d2::Config::builder().pool_size(size).build()
    } else {
        r2d2::Config::default()
    };
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("db pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub PooledConnection);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub mod schema;
