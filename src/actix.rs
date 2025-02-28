use crate::{activate_404_handler, actix_handle, data::init_args};
use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

activate_404_handler!();
actix_handle!(index, "index.html");

#[actix_web::main]
/// An example of implementation with static files
pub async fn init_server() -> std::io::Result<()> {
    let (port, ip) = init_args();
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(
                Files::new(&crate::FOLDER_HANDLER, &crate::FOLDER_HANDLER)
                    .index_file(include_str!(".config/404.html")),
            )
            .route("/index", web::get().to(index))
            .default_service(web::route().to(not_found))
    })
    .bind(format!("{}:{}", ip, port))
    .unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(-1)
    })
    .run()
    .await
}
