use std::borrow::BorrowMut;

use crate::db::DbConn;
use rocket::serde::json::{ Json, json };
use super::models::{Book, NewBook};
use rocket::serde::json::Value;

#[get("/books")]
pub fn index(mut conn: DbConn) -> Json<Value> {
    let result = Book::all(conn.borrow_mut());

    Json(json!({
        "status": 200,
        "result": result,
    }))
}

#[post("/books", format = "application/json", data = "<new_book>")]
pub fn new(mut conn: DbConn, new_book: Json<NewBook>) -> Json<Value> {
    Json(json!({
        "status": Book::insert(new_book.into_inner(), conn.borrow_mut()),
        "result": null,
    }))
}

#[get("/books/<id>")]
pub fn show(mut conn: DbConn, id: i32) -> Json<Value> {
    let result = Book::show(id, conn.borrow_mut());
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result.get(0),
    }))
}

#[put("/books/<id>", format = "application/json", data = "<book>")]
pub fn update(mut conn: DbConn, id: i32, book: Json<NewBook>) -> Json<Value> {
    let status = if Book::update_by_id(id, conn.borrow_mut(), book.into_inner()) { 200 } else { 404 };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[delete("/books/<id>")]
pub fn delete(mut conn: DbConn, id: i32) -> Json<Value> {
    let status = if Book::delete_by_id(id, conn.borrow_mut()) { 200 } else { 404 };

    Json(json!({
        "status": status,
        "result": null,
    }))
}

#[get("/books/author/<author>")]
pub fn author(mut conn: DbConn, author: String) -> Json<Value> {
    let result = Book::all_by_author(author, conn.borrow_mut());
    let status = if result.is_empty() { 404 } else { 200 };

    Json(json!({
        "status": status,
        "result": result,
    }))
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "result": "Resource was not found",
    }))
}
