## MEYRIN
![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![License](https://img.shields.io/github/license/joaoviictorti/RustRedOps)
</br>

An example of a simple, ready-to-use web server implementing actix

## HOW TO USE

```rust
// render index.html in static folder

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
                Files::new(&crate::FOLDER_HANDLER, &crate::FOLDER_HANDLER).index_file(include_str!(".config/404.html")),
            )
            .route("/index", web::get().to(index))
            .default_service(web::route().to(not_found))
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}

```
```bash
meyrin -p, --port <port> -i, --ip <ip address> -V, --Version <Print version>
```

### HOW BUILD
#### Requirements
```bash
cargo instal cargo-make
cargo install --locked cargo-zigbuild
```
[![asciicast](https://asciinema.org/a/a147ZvBMOQptYUqiAP2b8zmdx.svg)](https://asciinema.org/a/a147ZvBMOQptYUqiAP2b8zmdx)

#### NORMAL BUILD 
```bash
cargo make build
```
#### USING ZIGBUILD 
```shell
cargo make optimize
```
## DOCS
For large-scale and production projects, I recommend using Artix directly. You can consult the documentation here [[https://actix.rs/docs/]](https://actix.rs/docs/)