use diesel::pg::PgConnection;
use r2d2;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{Request, State, request::Outcome};
use std::ops::Deref;
use diesel::r2d2::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<DbConn, ()> {
        let pool = request.guard::<&State<Pool>>().await;
        match pool.succeeded() {
            Some(pool) => match pool.inner().get() {
                Ok(conn) => Outcome::Success(DbConn(conn)),
                Err(_) => Outcome::Error((Status::ServiceUnavailable, ())),
            },
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}