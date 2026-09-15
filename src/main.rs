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

    #[arg(short, long)]
    prometheus: bool,

    #[arg(long)]
    start_time: Option<String>,

    #[arg(long)]
    end_time: Option<String>,
}

fn main() {
    let args = Args::parse();

    let reader: Box<dyn BufRead> = match args.input {
        Some(path) => {
            let file = File::open(path).expect("Failed to open input file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut aggregator = MetricAggregator::new(&args.min_level);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }
                match LogEntry::parse(&line) {
                    Ok(entry) => {
                        if entry.matches_time_window(&args.start_time, &args.end_time) {
                            aggregator.process_entry(&entry);
                        }
                    }
                    Err(_) => {
                        // Skip malformed log lines silently or handle if strict mode enabled
                    }
                }
            }
            Err(_) => break,
        }
    }

    if args.prometheus {
        print!("{}", aggregator.generate_prometheus_output());
    } else if args.json_output {
        let report = aggregator.generate_report();
        let json = serde_json::to_string_pretty(&report).expect("Failed to serialize report");
        println!("{}", json);
    } else {
        let report = aggregator.generate_report();
        println!("=== Log Metrics Summary ===");
        println!("Total Processed: {}", report.total_processed);
        println!("Errors:          {}", report.errors);
        println!("Warnings:        {}", report.warnings);
        println!("P50 Latency:     {} ms", report.p50_latency);
        println!("P90 Latency:     {} ms", report.p90_latency);
        println!("P99 Latency:     {} ms", report.p99_latency);
        println!("Mean Latency:    {:.2} ms", report.mean_latency);
        println!("Max Latency:     {} ms", report.max_latency);
    }
}
