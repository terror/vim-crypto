use crate::common::*;

pub struct Table {
  table: PrettyTable,
}

impl Display for Table {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.table.to_string())
  }
}

impl Table {
  pub fn new() -> Self {
    let mut table = PrettyTable::new();

    table.set_format(*format::consts::FORMAT_BOX_CHARS);

    table.add_row(row![
      "Name",
      "Symbol",
      "Price $USD",
      "Price $BTC",
      "Change (24h)",
      "Volume (24h)",
    ]);

    Self { table }
  }

  pub fn add_row(&mut self, data: Vec<String>) {
    self
      .table
      .add_row(Row::new(data.iter().map(|item| Cell::new(item)).collect()));
  }
}
