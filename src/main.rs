use anyhow::Result;
use clap::Parser;
use cli_rust::{process_csv, CliOptions, Subcommand};

fn main() -> Result<()> {
    let parse = CliOptions::parse();
    match parse.cmd {
        Subcommand::Csv(csv) => {
            let output = if let Some(output) = csv.output {
                output.clone()
            } else {
                format!("output.{:?}", csv.format)
            };
            process_csv(&csv.input, output.as_str(), csv.format)?
        }
    }
    Ok(())
}
