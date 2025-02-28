use colored::*;
pub mod actix;
pub mod data;

pub static FOLDER_HANDLER: &'static str = "static";

fn ascii_init() {
    print!(
        "{}",
        "         •  
┏┳┓┏┓┓┏┏┓┓┏┓
┛┗┗┗ ┗┫┛ ┗┛┗
      ┛ server
"
        .purple()
    );
}

#[macro_export]
macro_rules! actix_handle {
    ($func_name : ident, $path : expr) => {
        async fn $func_name() -> impl Responder {
            let missing = include_str!(".config/404.html");
            let index =
                match std::fs::read_to_string(format!("{}/{}", crate::FOLDER_HANDLER, $path)) {
                    // TO_DO
                    Ok(ok) => ok,
                    Err(_) => missing.to_string(),
                };

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(index)
        }
    };
}

#[macro_export]
macro_rules! activate_404_handler {
    () => {
        async fn not_found() -> impl Responder {
            HttpResponse::NotFound()
                .content_type("text/html; charset=utf-8")
                .body(include_str!(".config/404.html"))
        }
    };
}
