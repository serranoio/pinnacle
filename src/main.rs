// use pinnacle::cli;
// use crate::cli::arxgs::PinnacleArgs;
pub mod cli;

use std::path::PathBuf;

use crate::cli::args::PinnacleArgs;
use clap::Parser;


use percy_dom::prelude::*;

use cli::State;

fn main() {
    let pinnacle_args: PinnacleArgs = PinnacleArgs::parse();
    
    let mut state = State::new();

    if let Err(e) = cli::process_project(pinnacle_args, &mut state) {
        eprintln!("Error from process_project: {}", e);
    }

    let component = html! { <div>{ "My text nodes here " }</div> };

    

    println!("{:?}", component);

}

