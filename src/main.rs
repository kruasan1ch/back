#[macro_use]
extern  crate diesel;


mod database;
mod models;
mod schema;
mod controllers;
mod router;

use actix_web::{App, middleware, HttpServer};
use actix_web::web::Data;
use actix_cors::Cors;

/// Точка входа в приложение.
/// Подтягивает хост и порт сервера из .env файла
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = std::env::var("HOST").expect("Hostname");
    let port = std::env::var("PORT").expect("Port");
    let address = format!("{}:{}",host,port);
    log::info!("Starting HTTP server at http://{}", &address);
    HttpServer::new(|| {
      let cors = Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin();
      App::new()
        .app_data(Data::new(database::init_pool().clone()))
        .wrap(middleware::Logger::default())
        .wrap(cors)
        .service(router::get_tasks)
        .service(router::add_task)
        .service(router::get_task)
        .service(router::delete_task)
        .service(router::update_task)
        .service(router::get_users)
        .service(router::add_user)
        .service(router::get_user)
        .service(router::delete_user)
        .service(router::update_user)
    }
      )
    .bind(address)?
    .run()
    .await
}
