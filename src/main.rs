#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

use dotenv::dotenv;
use rocket::{Build, Rocket};
use std::env;
use diesel::{prelude::*, result};
use diesel::pg::PgConnection;

mod schema;
mod models;
mod db;
mod static_files;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    rocket::build()
        .manage(pool)
        .mount("/", routes![static_files::index, static_files::all])
}
