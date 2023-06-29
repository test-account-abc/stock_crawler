use scraper::{Html, Selector};
use std::error::Error;

pub fn get_stock_value(document: String) -> Result<i32, Box<dyn Error>> {
    let html = Html::parse_document(&document);
    let amount_selector = Selector::parse(".kabuka").unwrap();
    for amount in html.select(&amount_selector) {
        let amount = convert_to_amount(amount.text().collect::<String>());
        return Ok(amount);
    }
    return Err("get_stock_value failed".to_string().into());
}

fn convert_to_amount(amount_str: String) -> i32 {
    let amount_str = amount_str.replace("円", "").replace(",", "");
    let amount: f32 = amount_str.parse().unwrap();
    return amount.round() as i32;
}
