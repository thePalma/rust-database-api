use diesel::{self};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;


#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, connection: &mut PgConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(connection)
            .expect("Error loading book")
    }

    pub fn all(connection: &mut PgConnection) -> Vec<Book> {
        all_books
            .load::<Book>(connection)
            .expect("Error loading books")
    }

    pub fn update_by_id(id: i32, connection: &mut PgConnection, book: NewBook) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};
        let NewBook {
            title,
            author,
            published,
        } = book;

        diesel::update(all_books.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .get_result::<Book>(connection)
            .is_ok()
    }

    pub fn insert(book: NewBook, connection: &mut PgConnection) -> bool {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(connection)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, connection: &mut PgConnection) -> bool {
        if Book::show(id, connection).is_empty() {
            return false;
        };
        diesel::delete(all_books.find(id))
            .execute(connection)
            .is_ok()
    }

    pub fn all_by_author(author: String, connection: &mut PgConnection) -> Vec<Book> {
        all_books
            .filter(books::author.eq(author))
            .load::<Book>(connection)
            .expect("Error loading books")
    }
}