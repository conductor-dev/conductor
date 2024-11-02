mod args;
mod error;
mod plotter;
mod voltmeter;

use args::{Args, Command};
use clap::Parser;
use error::ConductorSimResult;

fn main() -> ConductorSimResult<()> {
    let args = Args::parse();

    match args.command {
        Command::PeakVoltmeter(cmd) => voltmeter::voltmeter(cmd),
        Command::Plotter(cmd) => plotter::app::plotter(cmd),
    }
}
