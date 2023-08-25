pub mod args;

use std::path::PathBuf;

use args::{NewProject, PinnacleArgs, Subcommands, Build};

use std::path::Path;
use std::process::Command;
use lazy_static::lazy_static;

use std::fs::File;
use std::io::Write;

const WASM_PACK_INSTALL: &str = "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh";



fn create_build_command(state: &State) -> String {
    format!("wasm-pack build --target web --out-dir {}", state.directory_path.to_str().unwrap())
}

fn write_html_file(state: &mut State) -> String {
    format!("<!DOCTYPE html>
    <html lang='en'>
    <head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <title>Document</title>
    <link rel='stylesheet' href='./{}/pinnacle.css'>
    </head>
    <body>
        <script type='module'>
              import init, {{App }} from './{}/pinnacle.js'

                async function run ()  {{
                    await init('./{}/pinnacle_bg.wasm')
                    new App() 
                }}
    
                run()
        </script>
    </body>
    </html>", state.directory_path.display(), state.directory_path.display(), state.directory_path.display())
}

fn upsert_build(state: &mut State) {
    let command = create_build_command(&state);

    
    let output = Command::new("bash")
    .args([
        "-c",
        command.as_str()
        ])
        .output().expect("code ran");
    
    println!("{:?}", output);
}

// builds a new project, also can 
fn handle_new_project(new_project: NewProject, state: &mut State) -> std::io::Result<()> {
    // // let new_directory = &new_project.new_directory;
   
    
    state.directory_path.push(new_project.name);


    let mut html_file = File::create("index.html")?;

    html_file.write_all(write_html_file(state).as_bytes())?;
        

    
    Ok(())
}

fn handle_build(build: Build, state: &mut State) {
    upsert_build(state);

    
}


// downloads wasm pack
fn handle_download() {
    let _output = Command::new("bash")
            .args([
                "-c",
                WASM_PACK_INSTALL,
            ])
            .output().expect("code ran");

    println!("Downloaded all");
}


#[derive(Clone)]
pub struct State {
    directory_path: PathBuf,
}

impl State {
    pub fn new() -> State {
        State {directory_path: PathBuf::new()}
    }
}


pub fn process_project(pinnacle_args: PinnacleArgs, state: &mut State) -> std::io::Result<()> {
    match pinnacle_args.commands {
        Subcommands::New(new_project) => handle_new_project(new_project, state)?,
        Subcommands::Build(build) => {

            // let state = state.clone();
            handle_build(build, state);
            
        }
        Subcommands::Download => {
            handle_download();
            ()
        }
    };

    Ok(())
}
