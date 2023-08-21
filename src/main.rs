use clap::Parser;
use pinnacle::cli;
use crate::cli::args::PinnacleArgs;

fn main() {
    let pinnacle_args: PinnacleArgs = PinnacleArgs::parse();

    if let Err(e) = cli::process_project(pinnacle_args) {
        eprintln!("Error from process_project: {}", e);
    }
}
