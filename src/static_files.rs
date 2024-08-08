use std::io;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[get("/")]
pub async fn index() -> io::Result<NamedFile> {
    NamedFile::open("public/index.html").await
}

#[get("/<file..>")]
pub async fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).await.ok()
}