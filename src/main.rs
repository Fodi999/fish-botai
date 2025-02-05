mod chat_actor;
mod dialog;
mod handlers;
mod models;
mod nlp;

use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Запуск сервера на 0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            // Инициализируем маршруты для WebSocket
            .configure(handlers::init_routes)
            // Обслуживаем статические файлы из каталога "static"
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}