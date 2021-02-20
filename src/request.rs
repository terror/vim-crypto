use crate::common::*;

#[tokio::main]
pub async fn fetch() -> Result<String, Box<dyn error::Error>> {
  let resp = reqwest::get("https://data.messari.io/api/v2/assets?fields=id,slug,symbol,metrics")
    .await?
    .text()
    .await?;
  Ok(resp)
}
