//mod api;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::task::{get_task};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new( move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(get_task)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
