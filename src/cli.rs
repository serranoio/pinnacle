pub mod args;

use args::{Build, NewProject, PinnacleArgs, Subcommands};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const WASM_PACK_INSTALL: &str = "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh";

fn handle_html_file(directory_path: &Path) -> std::io::Result<File> {
    let mut html_file = File::create(directory_path.join("index.html"))?;

    Ok(html_file)
}

fn handle_js_file(directory_path: &Path) -> std::io::Result<File> {
    let mut js_file = File::create(directory_path.join("index.ts"))?;

    js_file.write(b"console.log('Hello, World!');")?;

    Ok(js_file)
}

fn handle_new_project(new_project: NewProject) -> std::io::Result<()> {
    // // let new_directory = &new_project.new_directory;
    let directory_path = Path::new(&new_project.name);

    println!("{}", directory_path.display());
    // // create directory
    // fs::create_dir(format!("{}", directory_path.display()))?;
    // // now we have to create js insertion file le
    let output = Command::new("ash")
    .args([
        "-c",
        format!("wasm-pack build --target web --out-dir {}", directory_path.display()).as_str(),
    ])
    .output().expect("code ran");



    Ok(())
}

fn handle_new_build(new_build: Build) {
    println!("pinnacle build()");
    // // start of pinnacle build
    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute process")
    // };

    // let hello = output.stdout;
}


/// this aint it
fn handle_download() {
    let output = Command::new("bash")
            .args([
                "-c",
                WASM_PACK_INSTALL,
            ])
            .output().expect("code ran");

    println!("Downloaded all");
}

pub fn process_project(pinnacle_args: PinnacleArgs) -> std::io::Result<()> {
    match pinnacle_args.commands {
        Subcommands::New(new_project) => handle_new_project(new_project)?,
        Subcommands::Build(new_build) => {
            handle_new_build(new_build);
            ()
        }
        Subcommands::Download => {
            handle_download();
            ()
        }
    };

    Ok(())
}
