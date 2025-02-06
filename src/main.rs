use actix::prelude::*;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use actix_web_actors::ws;
use shuttle_actix_web::ShuttleActixWeb;

// Определяем актора для работы с WebSocket‑соединением.
struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    // При установке соединения отправляем подробное приветственное сообщение.
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("Welcome to Chat Bot v1.1!\n\
                  This chat bot echoes your messages and responds to greetings.\n\
                  Try sending 'hello' or 'hi' to receive a friendly greeting, or type any message to see it echoed back.");
    }
}

// Реализуем обработчик входящих сообщений WebSocket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Отвечаем на Ping сообщением Pong.
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // При получении текстового сообщения отправляем эхо‑ответ и, при необходимости, приветствие.
            Ok(ws::Message::Text(text)) => {
                // Эхо‑ответ: отправляем сообщение с префиксом "Echo:".
                ctx.text(format!("Echo: {}", text));

                // Если сообщение содержит приветствие, отправляем дополнительное сообщение.
                let lower = text.to_lowercase();
                if lower.contains("hello") || lower.contains("hi") {
                    ctx.text("Hello! I'm your friendly chat bot. How can I assist you today?");
                }
            }
            // Если получено бинарное сообщение – отправляем его обратно.
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // При закрытии соединения – закрываем контекст актора.
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

// Маршрут, который инициирует WebSocket‑соединение.
#[get("/ws/")]
async fn websocket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket, &req, stream)
}

// Маршрут для вывода информации о чат‑боте и маршруте подключения для внешних сайтов.
#[get("/")]
async fn hello_world() -> impl Responder {
    let info = "\
Chat Bot Server v1.1
=====================
This server provides a WebSocket endpoint for chat communication.

WebSocket Endpoint (for external integration):
    wss://fish-botai-ye1g.shuttle.app/ws/

Instructions:
- To integrate the chat bot with your site, open a WebSocket connection to the URL above.
- The chat bot echoes all your messages with an 'Echo:' prefix.
- Additionally, it responds with a friendly greeting if your message contains 'hello' or 'hi'.

Enjoy chatting!
";
    HttpResponse::Ok().content_type("text/plain").body(info)
}

// Главная функция, которая конфигурирует Actix Web через Shuttle.
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(websocket_route);
    };

    Ok(config.into())
}








