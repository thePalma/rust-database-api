#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let connection = &mut PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("The Catcher in the Rye"),
        author: String::from("J.D. Salinger"),
        published: true,
    };

    if models::Book::insert(book, connection) {
        println!("Book inserted");
    } else {
        println!("Error inserting book");
    }
}
