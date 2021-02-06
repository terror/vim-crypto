use crate::common::*;

pub struct Crypto;

#[derive(Deserialize)]
struct Response {
    data: Vec<Data>,
}

#[derive(Deserialize)]
struct Data {
    slug: String,
    symbol: String,
    metrics: Metrics,
}

#[derive(Deserialize)]
struct Metrics {
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    price_usd: f64,
}

impl Crypto {
    pub fn new() -> Crypto {
        Crypto {}
    }

    pub fn parse(&self, res: String) -> serde_json::Result<String> {
        let resp: Response = serde_json::from_str(&res)?;

        let mut table = Table::new();
        table.add_row(row!["Name", "Symbol", "Price $USD"]);

        for data in resp.data {
            table.add_row(row![
                data.slug.to_uppercase(),
                data.symbol,
                format!("${:.2}", data.metrics.market_data.price_usd)
            ]);
        }
        Ok(table.to_string())
    }
}
