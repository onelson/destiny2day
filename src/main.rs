#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate d2_api;
mod db;

use diesel::result::QueryResult;
use rocket_contrib::json::Json;
use diesel::result::Error::DatabaseError as DbError;


#[get("/activities")]
fn activities(conn: db::DbConn) {
    unimplemented!();
}


fn main() {
    rocket::ignite()
        .manage(db::init_pool(None))
        .mount("/", routes![activities])
        .launch();
}
