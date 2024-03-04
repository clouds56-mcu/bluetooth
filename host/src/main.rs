use anyhow::Result;

pub mod core;
// pub mod ui;

fn main() -> Result<()> {
  core::demo()?;
  Ok(())
}
