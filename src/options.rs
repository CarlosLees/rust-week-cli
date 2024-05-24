use anyhow::anyhow;
use clap::Parser;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;

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
}

#[derive(Debug, Parser)]
pub struct CsvOptions {
    #[arg(short,long,value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short,long,default_value = "json", value_parser = verify_format)]
    pub format: Format,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

#[derive(Debug, Parser)]
pub struct GenPassOptions {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_format(format: &str) -> Result<Format, anyhow::Error> {
    format.parse()
}

#[derive(Debug, Copy, Clone)]
pub enum Format {
    Json,
    Toml,
    Yaml,
}

impl From<Format> for &str {
    fn from(value: Format) -> Self {
        match value {
            Format::Json => "json",
            Format::Toml => "toml",
            Format::Yaml => "yaml",
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl FromStr for Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(Format::Json),
            "toml" => Ok(Format::Toml),
            "yaml" => Ok(Format::Yaml),
            _ => Err(anyhow!("不支持此类型")),
        }
    }
}

fn verify_input_file(file: &str) -> anyhow::Result<String, &'static str> {
    if Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("文件不存在")
    }
}
