use crate::cli::csv::verify_input_file;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "encode")]
    Encode(Base64EncodeOption),
    #[command(name = "decode", about = "decode")]
    Decode(Base64DecodeOption),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOption {
    #[arg(short,long,default_value = "-", value_parser = verify_input_file)]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOption {
    #[arg(short, long)]
    pub input: String,
}
