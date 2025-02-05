pub enum NLPResult {
    Product(String),
    Unknown,
}

pub fn process_nlp(text: &str) -> NLPResult {
    let lower_text = text.to_lowercase();
    let products = [
        "креветки", "лосось", "тунец", "устрицы",
        "мидии", "краб", "осьминог", "кальмары",
    ];
    for &product in products.iter() {
        if lower_text.contains(product) {
            return NLPResult::Product(product.to_string());
        }
    }
    NLPResult::Unknown
}