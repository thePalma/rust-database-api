extern crate diesel;
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
use routes::*;

mod schema;
mod models;
mod db;
mod static_files;
mod routes;


#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url);

    rocket::build()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        )
        .mount("/", routes![static_files::all, static_files::index])
        .register("/", catchers![not_found])
}
