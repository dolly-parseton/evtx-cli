extern crate serde_json;
// use std::borrow::Cow;
use std::{
    io::{stderr, stdout, Write},
    path::PathBuf,
};
use structopt::StructOpt;
// use tau_engine::Rule;

#[derive(StructOpt)]
#[structopt(
    name = "evtx-cli",
    about = "Read one or more evtx files and apply some simple filters. Output is JSON"
)]
struct Opt {
    /// Provide one or more paths to valid `evtx` files.
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
    /// Enables attribute flattening, enabling this flattens objects containing '#attributes' fields into 'root_field_attributes'.
    #[structopt(short, long = "flatten", short)]
    flatten_attributes: bool,
}

fn main() {
    let mut opt = Opt::from_args();
    let mut stdout = stdout();
    let mut stderr = stderr();
    while let Some(path) = opt.files.pop() {
        // println!("{:?}", path);
        let mut parser = match evtx::EvtxParser::from_path(&path) {
            Ok(p) => p.with_configuration(
                evtx::ParserSettings::new().separate_json_attributes(opt.flatten_attributes),
            ),
            // If EvtxParser cannot be created then just ignore the file and print the error to stderr
            Err(e) => {
                eprintln!(
                    "Could not read event logs from {}, error: {}",
                    path.display(),
                    e
                );
                // Move to next file.
                continue;
            }
        };
        for record in parser.records_json_value() {
            match record {
                Ok(r) => match serde_json::to_string(&r.data) {
                    Ok(s) => {
                        writeln!(stdout, "{}", s);
                        // println!("{}", s);
                    }
                    Err(e) => writeln!(stderr, "{}", e),
                },
                Err(e) => {
                    writeln!(stderr, "{}", e);
                }
            }
        }
    }
}
