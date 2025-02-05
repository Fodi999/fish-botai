use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;
use crate::chat_actor::MyWebSocket;

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

fn validate_token(token: &str) -> bool {
    // В данном примере токен корректен, если равен "secret123"
    token == "secret123"
}

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    query: web::Query<WsQuery>,
) -> Result<HttpResponse, Error> {
    let token = query.into_inner().token;
    if !validate_token(&token) {
        return Ok(HttpResponse::Unauthorized().body("Неверный токен"));
    }
    ws::start(MyWebSocket::new(token), &req, stream)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(websocket_route)));
}