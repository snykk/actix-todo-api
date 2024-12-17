use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod config;
mod handlers;
mod models;
mod repositories;
mod schema;
mod utils;

use crate::config::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database = Database::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            // Root scope
            .service(web::scope("/api")
                // Health Check Endpoint
                .service(handlers::root::health_check)
                
                // Auth Routes: /api/auth/...
                .service(web::scope("/auth")
                    .service(handlers::auth::register)
                    .service(handlers::auth::login)
                )

                // Todo Routes: /api/todos/...
                .service(web::scope("/todos")
                    .service(handlers::todo::get_todos)
                    .service(handlers::todo::create_todo)
                    .service(handlers::todo::update_todo)
                    .service(handlers::todo::delete_todo)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
