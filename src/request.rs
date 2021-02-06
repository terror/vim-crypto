use crate::common::*;

#[tokio::main]
pub async fn fetch() -> Result<String, Box<dyn error::Error>> {
    let resp = reqwest::get(
        "https://data.messari.io/api/v1/assets?fields=id,slug,symbol,metrics/market_data/price_usd",
    )
    .await?
    .text()
    .await?;
    Ok(resp)
}
