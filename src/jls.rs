use std::fs;
use std::process;
use std::io;
use fs::DirEntry;
use clap::Parser;
use serde_json::json;

#[derive(Parser)]
#[clap(name = "jls", version = "1.0.0", author = "Tristan Wiesepape")]
struct Args {
    /// Directory to search
    directory: Option<String>,

    #[clap(long, short, action)]
    /// Pretty print json
    pretty_print: bool,

    // TODO
    /*
    #[clap(long, short, action, help="Do not ignore entries starting with .")]
    all: bool,
    */

    #[clap(long, short='A', action)]
    /// Do not ignore entries starting with . except for . and ..
    almost_all: bool,
}

fn handle_dir_entry(entry: DirEntry) -> serde_json::Value {
    // represent invalid bytes in filenames with U+FFFD REPLACEMENT CHARACTER
    return json!({"filename": entry.file_name().to_string_lossy().to_string()});
}

fn process(args: Args) -> io::Result<()> {
    let directory = args.directory.unwrap_or("./".to_string());

    let paths_with_errors: Vec<io::Result<DirEntry>> = fs::read_dir(&directory)?.collect();

    if let Some(index) = paths_with_errors.iter().position(|r| r.is_err()) {
        let error = paths_with_errors.get(index).expect("Safe because index was obtained from position()").as_ref().unwrap_err();
        // TODO copy error properly
        return Err(io::Error::new(error.kind(), format!("{}", error)));
    }

    let mut paths: Vec<DirEntry> = paths_with_errors
                                   .into_iter()
                                   .map(|r| r.expect("Safe because we know there are no errors"))
                                   .collect();


    if !args.almost_all {
         paths = paths.into_iter().filter(|path| {
             path.file_name().to_string_lossy().chars().nth(0).expect("Safe because filenames cannot be zero length") != '.'
        }).collect();
    }

    let data: Vec<serde_json::Value> = paths.into_iter()
        .map(handle_dir_entry)
        .collect();

    if args.pretty_print {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }
    else {
        println!("{}", serde_json::to_string(&data).unwrap());
    }

    Ok(())
}

fn main() {
    if let Err(err) = process(Args::parse()) {
        eprintln!("jls: {}", err);
        process::exit(err.raw_os_error().unwrap_or(1));
    }

}
