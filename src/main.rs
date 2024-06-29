//TODO: Add Build Tooling for each template :ODOT//

use clap::Parser;
use std::{env, fs, io, path::Path};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which languages template
    #[arg(short, long)]
    template: String,
    /// path to output to.
    #[arg(short, long)]
    output_path: String,
}
/// Copy files from source to destination recursively.
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
    let source = source.as_ref();
    let destination = destination.as_ref();

    // Create destination directory if it doesn't exist
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_entry_path = entry.path();
        let destination_entry_path =
            destination.join(source_entry_path.strip_prefix(source).unwrap());

        if entry.file_type()?.is_dir() {
            copy_recursively(source_entry_path, destination_entry_path)?;
        } else {
            fs::copy(source_entry_path, destination_entry_path)?;
        }
    }

    Ok(())
}
fn main() {
    let args = Args::parse();
    match args.template.as_str() {
        "c" => {
            let _ = copy_recursively("./templates/c/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "empty" => {
            let _ = copy_recursively("./templates/empty/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "go" => {
            let _ = copy_recursively("./templates/go/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "haskell" => {
            let _ = copy_recursively("./templates/haskell/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "js" => {
            let _ = copy_recursively("./templates/js/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "ocaml" => {
            let _ = copy_recursively("./templates/ocaml/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "python" => {
            let _ = copy_recursively("./templates/python/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "rust" => {
            let _ = copy_recursively("./templates/rust/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        "zig" => {
            let _ = copy_recursively("./templates/zig/", args.output_path);
            println!("cloning {} template!", args.template);
        }
        _ => {
            println!("there is no {} template available!", args.template);
        }
    }
}
