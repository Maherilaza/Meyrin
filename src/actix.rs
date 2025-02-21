use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::{actix_handle, data::init_args};

actix_handle!(index, "index.html");

#[actix_web::main]
pub async fn init_server() -> std::io::Result<()> {
    let (port, ip) = init_args();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/").route("", web::get().to(index)))
            .route("/index.html", web::get().to(index))
            .service(Files::new("/", "app").show_files_listing())
    })
    .bind((ip, port))?
    .run()
    .await
}