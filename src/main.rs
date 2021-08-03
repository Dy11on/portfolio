use actix_web::{Result,App,HttpRequest,web, HttpServer};
use actix_files::*;
use std::path::PathBuf;


async fn index(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = "./files/index.html".parse().unwrap();
    println!("{:?}", path);
    Ok(NamedFile::open(path)?)
}

async fn resume(_req: HttpRequest) -> Result<actix_files::NamedFile> {
    let path: PathBuf = "./files/resume.pdf".parse().unwrap();
    println!("{:?}", path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
                    App::new()
                    .route("/", web::get().to(index))
                    .route("/index.html", web::get().to(index))
                    .route("/resume", web::get().to(resume))
                    .service(Files::new("/static","./files").show_files_listing()))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
