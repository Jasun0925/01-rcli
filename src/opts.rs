use core::fmt;
use std::str::FromStr;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(
        short = 'l',
        long = "length",
        help = "Password length",
        default_value_t = 16
    )]
    pub length: u8,
    #[arg(long = "uppercase", help = "Include uppercase", default_value_t = true)]
    pub uppercase: bool,
    #[arg(long = "lowercase", help = "Include lowercase", default_value_t = true)]
    pub lowercase: bool,
    #[arg(
        short = 'n',
        long = "numbers",
        help = "Include numbers",
        default_value_t = true
    )]
    pub numbers: bool,
    #[arg(
        short = 's',
        long = "symbols",
        help = "Include symbols",
        default_value_t = true
    )]
    pub symbols: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long = "input", help = "Input CSV file", value_parser = verify_input_file)]
    pub input: String,
    #[arg(
        short = 'o',
        long = "output",
        help = "Output file",
        //default_value = "output.json"
    )]
    pub output: Option<String>,
    #[arg(long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    //pub delimiter: char,
    #[arg(short = 'H', long = "header", help = "Header", default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format: {}", s)),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self)
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
