use crate::common::*;

pub struct Crypto;

#[derive(Deserialize)]
struct Response {
  data: Vec<Data>,
}

#[derive(Deserialize)]
struct Data {
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
  price_usd: f64,
  price_btc: f64,
  price_eth: f64,
  real_volume_last_24_hours: f64,
  percent_change_usd_last_24_hours: f64,
  last_trade_at: String,
}

impl Crypto {
  pub fn new() -> Crypto {
    Crypto {}
  }

  pub fn parse(&self, res: String) -> serde_json::Result<String> {
    let resp: Response = serde_json::from_str(&res)?;

    let mut table = Table::new();
    table.add_row(row![
      "Name",
      "Symbol",
      "Price $USD",
      "Price $BTC",
      "Price $ETH",
      "Change (24h)",
      "Volume (24h)",
      "Last Traded"
    ]);

    for data in resp.data {
      let last_traded_at = NaiveDateTime::parse_from_str(
        &data.metrics.market_data.last_trade_at,
        "%Y-%m-%dT%H:%M:%S%Z",
      )
      .unwrap();

      let now = chrono::Utc::now().naive_utc();

      let diff = now.signed_duration_since(last_traded_at);

      table.add_row(row![
        data.slug.to_uppercase(),
        data.symbol,
        format!("${:.2}", data.metrics.market_data.price_usd),
        format!("{:.2}", data.metrics.market_data.price_btc),
        format!("{:.2}", data.metrics.market_data.price_eth),
        format!(
          "{:.2}%",
          data.metrics.market_data.percent_change_usd_last_24_hours
        ),
        format!("${:.2}", data.metrics.market_data.real_volume_last_24_hours),
        format!("{} seconds ago", diff.num_seconds())
      ]);
    }
    Ok(table.to_string())
  }
}
