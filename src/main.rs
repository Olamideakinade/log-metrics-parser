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
#[command(name = "log-metrics-parser", version = "1.4.0")]
struct Args {
    #[arg(short, long)]
    input: Option<PathBuf>,
    #[arg(short, long, default_value = "INFO")]
    min_level: String,
    #[arg(long)]
    start_time: Option<String>,
    #[arg(long)]
    end_time: Option<String>,
    #[arg(long)]
    prometheus: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let reader: Box<dyn BufRead> = match args.input {
        Some(ref path) => Box::new(BufReader::new(File::open(path)?)),
        None => Box::new(BufReader::new(io::stdin())),
    };

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} [{elapsed_precise}] Processing logs...")
            .unwrap(),
    );
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let mut aggregator = MetricAggregator::new(&args.min_level);

    for line_result in reader.lines() {
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(entry) = LogEntry::parse(&line) {
            if entry.matches_time_window(&args.start_time, &args.end_time) {
                aggregator.process_entry(&entry);
            }
        }
    }

    pb.finish_and_clear();

    let report = aggregator.finalize();

    if args.prometheus {
        print!("{}", aggregator.render_prometheus(&report));
    } else {
        println!("{}", style("=== Log Metrics Summary Report ===").bold().cyan());
        println!("Total Processed: {}", style(report.total_processed).green());
        println!("Errors:          {}", style(report.errors).red());
        println!("Warnings:        {}", style(report.warnings).yellow());
        println!("Mean Latency:    {:.2} ms", report.mean_latency);
        println!("P50 Latency:     {} ms", report.p50_latency);
        println!("P90 Latency:     {} ms", report.p90_latency);
        println!("P99 Latency:     {} ms", report.p99_latency);
        println!("Max Latency:     {} ms", report.max_latency);
    }

    Ok(()}
}
