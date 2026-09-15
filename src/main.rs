use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use console::style;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

mod aggregator;
mod parser;

use aggregator::MetricAggregator;
use parser::LogEntry;

#[derive(Parser, Debug)]
#[command(name = "log-metrics-parser", version = "1.3.0")]
struct Args {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(short, long, default_value = "INFO")]
    min_level: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let reader: Box<dyn BufRead> = match args.input {
        Some(path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
    pb.set_message("Parsing logs...");

    let mut aggregator = MetricAggregator::new(&args.min_level);
    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(entry) = LogEntry::parse(&l) {
                aggregator.process(entry);
            }
        }
    }

    pb.finish_with_message(style("Processing complete.").green().to_string());
    println!("{}", style("--- PERFORMANCE SUMMARY ---").bold().cyan());
    println!("{}: {}", style("Total Processed").dim(), aggregator.total_processed());
    Ok(()) 
}