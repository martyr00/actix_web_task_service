mod api;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::task::get_task;
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;

    HttpServer::new(move || {
        let ddb_repo: DDBRepository = DDBRepository::init(
            String::from("task"),
            config.clone()
        );
        let ddb_data = Data::new(
            ddb_repo
        );

        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
