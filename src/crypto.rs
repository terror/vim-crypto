use crate::common::*;

#[derive(Debug)]
pub struct Crypto;

#[derive(Deserialize)]
struct ResponseOne {
  data: DataOne,
}

#[derive(Deserialize)]
struct ResponseTop {
  data: Vec<DataTop>,
}

#[derive(Deserialize)]
struct DataOne {
  slug:        String,
  symbol:      String,
  market_data: MarketData,
}

#[derive(Deserialize)]
struct DataTop {
  slug:    String,
  symbol:  String,
  metrics: Metrics,
}

#[derive(Deserialize)]
struct Metrics {
  market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
  price_usd:                        f64,
  price_btc:                        f64,
  real_volume_last_24_hours:        f64,
  percent_change_usd_last_24_hours: f64,
}

impl Crypto {
  pub fn parse_top(res: String) -> Result<String, Error> {
    let resp: ResponseTop = serde_json::from_str(&res).context(error::ParseError)?;

    let mut table = Table::new();

    for data in resp.data {
      table.add_row(vec![
        data.slug.to_uppercase(),
        data.symbol,
        format!("${:.2}", data.metrics.market_data.price_usd),
        format!("{:.2}", data.metrics.market_data.price_btc),
        format!(
          "{:.2}%",
          data.metrics.market_data.percent_change_usd_last_24_hours
        ),
        format!("${:.2}", data.metrics.market_data.real_volume_last_24_hours),
      ]);
    }

    Ok(table.to_string())
  }

  pub fn parse_one(res: String) -> Result<String, Error> {
    let resp: ResponseOne = serde_json::from_str(&res).context(error::ParseError)?;

    let mut table = Table::new();

    table.add_row(vec![
      resp.data.slug.to_uppercase(),
      resp.data.symbol,
      format!("${:.2}", resp.data.market_data.price_usd),
      format!("{:.2}", resp.data.market_data.price_btc),
      format!(
        "{:.2}%",
        resp.data.market_data.percent_change_usd_last_24_hours
      ),
      format!("${:.2}", resp.data.market_data.real_volume_last_24_hours),
    ]);

    Ok(table.to_string())
  }
}
