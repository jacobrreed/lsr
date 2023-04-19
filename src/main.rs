use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The directory to list
    #[arg(short, long, default_value=".")]
    dir: String,

    /// Whether to show all files, including hidden ones
    #[arg(short, long, default_value="true")]
    all: bool,
}

fn main() {
    let args = Args::parse();

    // get all files and folders in directory
    let paths = fs::read_dir(&args.dir).unwrap();
    // then sort the paths
    let mut paths: Vec<_> = paths.collect();
    paths.sort_by(|a, b| {
        let a = a.as_ref().unwrap();
        let b = b.as_ref().unwrap();
        let a = a.file_name();
        let b = b.file_name();
        let a = a.to_str().unwrap();
        let b = b.to_str().unwrap();
        a.cmp(b)
    });
    // then get the file permissions for each path and store them in a tuple
    let paths = paths.into_iter().map(|path| {
        let path = path.unwrap();
        let path = path.path();
        let path = path.to_str().unwrap();
        let metadata = fs::metadata(path).unwrap();
        let permissions = metadata.permissions();
        (path.to_string(), permissions)
    });
    // if current directory remove the first `./` from the path
    // otherwise remove first `/` from the path
    let paths = paths.map(|(path, permissions)| {
        if path.starts_with("./") {
            (path[2..].to_string(), permissions)
        } else {
            (path[1..].to_string(), permissions)
        }
    });
    // if the permissions are read only, print the path in red and add '(readonly)' separated by two `\t`s to the end
    for (path, permissions) in paths {
        if permissions.readonly() {
            println!("\x1b[31m{}\x1b[0m\t\t(readonly)", path);
        } else {
            println!("{}", path);
        }
    }
  }