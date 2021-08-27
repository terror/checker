use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(
  name = "checker",
  about = "a crates.io crate name availability checker"
)]
pub struct Opt {
  /// Crate name(s)
  #[structopt(short = "c", long = "check", help = "Check crate name availability")]
  names: Vec<String>,

  /// Output file, defaults to stdout
  #[structopt(short = "o", long = "output", help = "Output file")]
  output: Option<PathBuf>,
}

impl Opt {
  pub fn run(&self) -> Result<()> {
    let mut table = Table::new();

    for name in &self.names {
      table.add_row(check(&name)?)?;
    }

    if let Some(output) = &self.output {
      write(&output, table.to_string())?;
      return Ok(());
    }

    table.print();
    Ok(())
  }
}
