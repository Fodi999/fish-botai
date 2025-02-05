use once_cell::sync::Lazy;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct DialogTexts {
    pub start: StartDialog,
    pub ask_name: AskNameDialog,
    pub ask_needs: AskNeedsDialog,
    pub answering: AnsweringDialog,
    pub completion: CompletionDialog,
    pub clarify: ClarifyDialog,
}

#[derive(Deserialize)]
pub struct StartDialog {
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct AskNameDialog {
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct AskNeedsDialog {
    pub prompt: String,
}

#[derive(Deserialize)]
pub struct AnsweringDialog {
    pub response_product: String,
    pub response_default: String,
}

#[derive(Deserialize)]
pub struct CompletionDialog {
    pub thanks: String,
}

#[derive(Deserialize)]
pub struct ClarifyDialog {
    pub not_understood: String,
}

// Загружаем тексты диалога один раз при старте приложения.
pub static DIALOG_TEXTS: Lazy<DialogTexts> = Lazy::new(|| {
    let file_content = fs::read_to_string("dialog.json")
        .expect("Не удалось прочитать файл dialog.json. Убедитесь, что он находится в корне проекта.");
    serde_json::from_str(&file_content)
        .expect("Ошибка парсинга JSON из файла dialog.json")
});