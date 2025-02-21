use crate::{activate_404_handler, actix_handle, data::init_args};
// use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

activate_404_handler!();
actix_handle!(index, "static/index.html");

#[actix_web::main]
pub async fn init_server() -> std::io::Result<()> {
    let (port, ip) = init_args();

    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(not_found))
            .route("/", web::get().to(index))
            //.service(Files::new("/", "static").show_files_listing())
            .route("/index", web::get().to(index))
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
