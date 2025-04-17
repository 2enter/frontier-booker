use scraper::{Html, Selector};
use std::error::Error;

const URL: &str = "https://www.cwa.gov.tw/V8/C/W/Observe/MOD/24hr/C0C63.html";

pub async fn is_raining() -> Result<bool, Box<dyn Error>> {
    let html = reqwest::get(URL).await?.text().await?;
    let fragment = Html::parse_fragment(&html);
    let selector = Selector::parse("img")?;
    if let Some(first_image) = fragment.select(&selector).next() {
        Ok(first_image.html().contains("雨"))
    } else {
        Ok(false)
    }
}
