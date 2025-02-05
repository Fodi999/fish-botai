use actix::prelude::*;
use actix_web_actors::ws;
use std::fs;
use std::io::ErrorKind;
use bincode;
use serde_json;
use crate::dialog::DIALOG_TEXTS;
use crate::models::{BotMessage, ChatEntry, current_timestamp};
use crate::nlp::{process_nlp, NLPResult};

#[derive(Debug)]
pub enum DialogState {
    Start,
    AskName,
    AskNeeds(String),
    Answering(String),
    Completed,
}

pub struct MyWebSocket {
    pub state: DialogState,
    pub token: String,
    pub history: Vec<ChatEntry>,
}

impl MyWebSocket {
    pub fn new(token: String) -> Self {
        let filename = format!("chat_history_{}.bin", token);
        let history = match fs::read(&filename) {
            Ok(bytes) => bincode::deserialize(&bytes).unwrap_or_default(),
            Err(ref e) if e.kind() == ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                eprintln!("Ошибка чтения файла {}: {:?}", filename, e);
                Vec::new()
            }
        };
        Self {
            state: DialogState::Start,
            token,
            history,
        }
    }

    pub fn persist_history(&self) {
        let filename = format!("chat_history_{}.bin", self.token);
        match bincode::serialize(&self.history) {
            Ok(bytes) => {
                if let Err(e) = fs::write(&filename, bytes) {
                    eprintln!("Ошибка записи в файл {}: {:?}", filename, e);
                }
            }
            Err(e) => {
                eprintln!("Ошибка сериализации истории: {:?}", e);
            }
        }
    }

    pub fn add_history(&mut self, sender: &str, content: &str) {
        let entry = ChatEntry {
            sender: sender.to_owned(),
            content: content.to_owned(),
            timestamp: current_timestamp(),
        };
        self.history.push(entry);
        self.persist_history();
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let response = BotMessage {
            message_type: "start".to_owned(),
            content: DIALOG_TEXTS.start.prompt.clone(),
        };
        let response_text = serde_json::to_string(&response).unwrap();
        ctx.text(response_text);
        self.add_history("bot", &response.content);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Токен: {}. Получено сообщение: {}", self.token, text);
                self.add_history("user", &text);

                let response = match &mut self.state {
                    DialogState::Start => {
                        if text.to_lowercase().contains("привет") {
                            self.state = DialogState::AskName;
                            BotMessage {
                                message_type: "ask_name".to_owned(),
                                content: DIALOG_TEXTS.ask_name.prompt.clone(),
                            }
                        } else {
                            BotMessage {
                                message_type: "error".to_owned(),
                                content: DIALOG_TEXTS.clarify.not_understood.clone(),
                            }
                        }
                    }
                    DialogState::AskName => {
                        let name = text.trim().to_owned();
                        self.state = DialogState::AskNeeds(name.clone());
                        BotMessage {
                            message_type: "ask_needs".to_owned(),
                            content: DIALOG_TEXTS.ask_needs.prompt.replace("{name}", &name),
                        }
                    }
                    DialogState::AskNeeds(name) => {
                        match process_nlp(&text) {
                            NLPResult::Product(product) => {
                                self.state = DialogState::Answering(name.clone());
                                BotMessage {
                                    message_type: "answering".to_owned(),
                                    content: DIALOG_TEXTS.answering.response_product.replace("{product}", &product),
                                }
                            }
                            NLPResult::Unknown => {
                                BotMessage {
                                    message_type: "clarify".to_owned(),
                                    content: DIALOG_TEXTS.clarify.not_understood.clone(),
                                }
                            }
                        }
                    }
                    DialogState::Answering(_name) => {
                        let lower_text = text.to_lowercase();
                        if lower_text.contains("да") || lower_text.contains("спасибо") {
                            self.state = DialogState::Completed;
                            BotMessage {
                                message_type: "completed".to_owned(),
                                content: DIALOG_TEXTS.completion.thanks.clone(),
                            }
                        } else {
                            BotMessage {
                                message_type: "answering".to_owned(),
                                content: DIALOG_TEXTS.answering.response_default.clone(),
                            }
                        }
                    }
                    DialogState::Completed => BotMessage {
                        message_type: "completed".to_owned(),
                        content: DIALOG_TEXTS.completion.thanks.clone(),
                    },
                };

                let response_text = serde_json::to_string(&response).unwrap();
                ctx.text(response_text);
                self.add_history("bot", &response.content);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                println!("Соединение закрыто: {:?}", reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}