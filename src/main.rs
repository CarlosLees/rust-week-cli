use anyhow::Result;
use clap::Parser;
use cli_rust::process::process_gen_pass;
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
        Subcommand::GenPass(gen_pass) => {
            let password = process_gen_pass(
                gen_pass.length,
                gen_pass.uppercase,
                gen_pass.lowercase,
                gen_pass.number,
                gen_pass.symbol,
            )?;
            println!("{:?}", password)
        }
    }
    Ok(())
}
