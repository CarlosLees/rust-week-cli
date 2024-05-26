use crate::cli::base64::Base64Subcommand;
use crate::cli::csv::CsvOptions;
use crate::cli::gen_pass::GenPassOptions;
use clap::Parser;

pub mod base64;
pub mod csv;
pub mod gen_pass;

// cli csv -i input.csv -0 output.json
#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about, long_about = None)]
pub struct CliOptions {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "把csv转化为json")]
    Csv(CsvOptions),
    #[command(name = "gen_pass", about = "生成随机密码")]
    GenPass(GenPassOptions),
    #[command(subcommand)]
    Base64(Base64Subcommand),
}
