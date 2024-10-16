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
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long = "input", help = "Input CSV file", value_parser = verify_input_file)]
    pub input: String,
    #[arg(
        short = 'o',
        long = "output",
        help = "Output file",
        default_value = "output.json"
    )]
    pub output: String,
    #[arg(
        short = 'd',
        long = "delimiter",
        help = "Delimiter",
        default_value_t = ','
    )]
    pub delimiter: char,
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
