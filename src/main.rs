use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

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
            let file = File::open(path).unwrap_or_else(|err| {
                eprintln!("Error opening file: {}", err);
                std::process::exit(1);
            });
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut aggregator = MetricAggregator::new(&args.min_level);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match LogEntry::parse(trimmed) {
            Ok(entry) => aggregator.ingest(&entry),
            Err(_err) => {
                // Silently skip or log parse errors depending on production strictness
            }
        }
    }

    let report = aggregator.finalize();

    if args.json_output {
        let json_data = serde_json::to_string_pretty(&report).unwrap();
        println!("{}", json_data);
    } else {
        println!("=== Log Metrics Summary Report ===");
        println!("Total Processed : {}", report.total_processed);
        println!("Errors / Fatal  : {}", report.errors);
        println!("Warnings        : {}", report.warnings);
        println!("P50 Latency     : {} ms", report.p50_latency);
        println!("P90 Latency     : {} ms", report.p90_latency);
        println!("P99 Latency     : {} ms", report.p99_latency);
        println!("==================================");
    }
}
