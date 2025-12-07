use clap::CommandFactory;
use std::env;
use std::fs;
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    // Generate man page
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir).unwrap();

    let cmd = cli::Args::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();

    let man_path = man_dir.join("tp.1");
    fs::write(&man_path, buffer).unwrap();

    // Also copy to project root for easy access
    let project_man_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("man");
    fs::create_dir_all(&project_man_dir).unwrap();
    fs::copy(&man_path, project_man_dir.join("tp.1")).unwrap();

    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=build.rs");
}
