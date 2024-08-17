mod args;
mod error;
mod voltmeter;

use args::Args;
use clap::Parser;
use error::ConductorSimResult;

fn main() -> ConductorSimResult<()> {
    let args = Args::parse();

    match args.command {
        args::Command::PeakVoltmeter(cmd) => voltmeter::voltmeter(cmd),
    }
}
