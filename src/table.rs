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

    table.add_row(row!["Name", "Status", "Owner(s)", "Last Updated",]);

    Self { table }
  }

  pub fn add_row(&mut self, package: Package) -> Result<()> {
    match package.status {
      Status::Free => {
        self.table.add_row(row![
          package.name,
          Fg -> &package.status.to_string(),
          "N/A",
          "N/A"
        ]);
      }
      Status::Taken => {
        let days = &package.days_since_last_updated()?;

        let mut owners = String::new();
        if let Some(package_owners) = &package.owners {
          for owner in package_owners {
            owners.push_str(&owner.to_string());
          }
        }

        self.table.add_row(row![
          package.name,
          Fr -> &package.status.to_string(),
          owners,
          format!("{} day(s) ago", days),
        ]);
      }
      _ => {}
    }

    Ok(())
  }

  pub fn print(&self) {
    self.table.printstd();
  }
}
