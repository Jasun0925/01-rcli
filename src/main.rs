use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{process_csv, Opts, SubCommand};

// cargo run -- csv -i assets/juventus.csv
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
