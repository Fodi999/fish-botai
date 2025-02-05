mod chat_actor;
mod dialog;
mod handlers;
mod models;
mod nlp;

use actix_files::Files;
use actix_web::{App, HttpServer};
use std::thread;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Запуск сервера на 0.0.0.0:8080");

    // Запускаем сервер
    let server = HttpServer::new(|| {
        App::new()
            // Инициализируем маршруты для WebSocket
            .configure(handlers::init_routes)
            // Обслуживаем статические файлы из каталога "static"
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run();

    // Параллельно запускаем задачу, которая подождёт 1 секунду и откроет браузер.
    thread::spawn(|| {
        // Ждём немного, чтобы сервер точно запустился
        thread::sleep(Duration::from_secs(1));
        if webbrowser::open("http://localhost:8080").is_ok() {
            println!("Браузер успешно открыт!");
        } else {
            println!("Не удалось открыть браузер.");
        }
    });

    // Ожидаем завершения работы сервера (он будет работать до его остановки)
    server.await
}
