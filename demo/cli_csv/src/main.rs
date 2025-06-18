use clap::Parser;
use cli_csv::{Subcommand, Opts, process_csv};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) =>{
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
