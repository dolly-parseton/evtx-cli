extern crate serde;
extern crate serde_json;
use afrs::Rule;
use std::{fs, io::BufReader, path::PathBuf};
use structopt::StructOpt;

mod pattern_matching;

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
    #[structopt(long = "flatten", short)]
    flatten_attributes: bool,
    /// Match on one or more AFRS rules.
    #[structopt(long, short, parse(from_os_str))]
    rules: Option<Vec<PathBuf>>,
}

fn main() {
    let mut opt = Opt::from_args();
    let mut rules = Vec::new();
    if let Some(rules_) = opt.rules {
        for rule in rules_ {
            let f = fs::File::open(rule).unwrap();
            let r = BufReader::new(f);
            let rule: Rule = serde_json::from_reader(r).unwrap();
            rules.push(rule.validate().unwrap());
        }
    }
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
                        // Match on the rule if there
                        if rules.is_empty() {
                            println!("{}", s);
                        } else {
                            for rule in &rules {
                                if rule.match_json(&s) {
                                    println!("Matched on: {} \n{}", "", s);
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
}
