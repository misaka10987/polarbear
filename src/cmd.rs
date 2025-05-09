use clap::Parser;

use crate::Config;

#[derive(Parser)]
pub enum SubCommand {
    /// Print the default configuration to console.
    PrintConfig,
}

impl SubCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            SubCommand::PrintConfig => print_config(),
        }
    }
}

fn print_config() -> anyhow::Result<()> {
    let cfg = Config::default();
    let toml = toml::to_string_pretty(&cfg)?;
    println!("{toml}");
    Ok(())
}
