use crate::common::*;

#[derive(Debug)]
pub struct Client {
  base_url: String,
  client:   blocking::Client,
}

impl Client {
  pub fn new() -> Self {
    Self {
      base_url: String::from("https://data.messari.io/api/v1/assets"),
      client:   blocking::Client::new(),
    }
  }

  pub fn get(&self) -> Result<String, Error> {
    let url = &self.base_url;

    let response = self
      .client
      .get(url)
      .send()
      .context(error::RequestError { url })?;

    let text = response.text().context(error::RequestError { url })?;

    Ok(text)
  }

  pub fn get_one(&self, symbol: &str) -> Result<String, Error> {
    let url = &format!("{}/{}/metrics", self.base_url, symbol);

    let response = self
      .client
      .get(url)
      .send()
      .context(error::RequestError { url })?;

    let text = response.text().context(error::RequestError { url })?;

    Ok(text)
  }
}
