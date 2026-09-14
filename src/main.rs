use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process;

mod aggregator;
mod parser;

use aggregator::MetricAggregator;
use parser::LogEntry;

#[derive(Parser, Debug)]
#[command(name = "log-metrics-parser")]
#[command(about = "Parses structured logs and computes aggregate performance metrics.", long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    #[arg(short, long, default_value = "INFO")]
    min_level: String,

    #[arg(short, long)]
    json_output: bool,
}

fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.input {
        Some(path) => {
            let file = File::open(path).unwrap_or_else(|err {
                eprintln!("Error opening file: {}", err);
                process::exit(1);
            });
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut aggregator = MetricAggregator::new(&args.min_level);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        match LogEntry::parse(&line) {
            Ok(entry) => aggregator.ingest(entry),
            Err(err) => {
                eprintln!("Parse error: {} [Line: {}]", err, line);
            }
        }
    }

    if args.json_output {
        aggregator.print_json();
    } else {
        aggregator.print_text();
    }
}
