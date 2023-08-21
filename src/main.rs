mod args;

use args::{PinnacleArgs, NewProject, Subcommands};
use clap::Parser;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;

fn handle_html_file(directory_path: &Path) -> std::io::Result<File> {
    let mut html_file = File::create(directory_path.join("index.html"))?;





    Ok(html_file)
}

fn handle_js_file(directory_path: &Path) -> std::io::Result<File> {
    let mut js_file = File::create(directory_path.join("index.js"))?;

    js_file.write(b"console.log('Hello, World!');")?;

    Ok(js_file)
}

fn handle_new_project(new_project: NewProject) -> std::io::Result<()> {
    // let new_directory = &new_project.new_directory;
    let directory_path = Path::new(&new_project.name);
    // create directory
    fs::create_dir(format!("{}", directory_path.display()))?;
    // now we have to create js insertion file le
    handle_html_file(directory_path)?;
    handle_js_file(directory_path)?;

    Ok(())
}

fn process_project(pinnacle_args: PinnacleArgs) -> std::io::Result<()> {
    match pinnacle_args.commands {
        Subcommands::New(new_project) => handle_new_project(new_project)?, 
    };

    Ok(())
}

fn main() {
    let pinnacle_args: PinnacleArgs = PinnacleArgs::parse();

    if let Err(e) = process_project(pinnacle_args) {
        eprintln!("Error from process_project: {}", e);
    }
}