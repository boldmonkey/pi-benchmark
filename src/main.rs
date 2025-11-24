use std::env;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_LEIBNIZ_ITERATIONS: u64 = 50_000_000;
const DEFAULT_MONTE_CARLO_SAMPLES: u64 = 200_000_000;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || matches_help(&args[0]) {
        print_global_usage();
        return;
    }

    let mode = args.remove(0);

    let result = match mode.as_str() {
        "single" | "leibniz" => run_single_threaded(&args),
        "monte" | "monte-carlo" | "multi" | "multi-thread" => run_monte_carlo(&args),
        _ => {
            eprintln!("Unknown mode: {mode}\n");
            print_global_usage();
            return;
        }
    };

    if let Err(err) = result {
        eprintln!("{err}\n");
        print_mode_usage(mode.as_str());
        std::process::exit(1);
    }
}

fn run_single_threaded(args: &[String]) -> Result<(), String> {
    let mut iterations = DEFAULT_LEIBNIZ_ITERATIONS;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--iterations" | "-n" => {
                i += 1;
                let value = args.get(i).ok_or("Missing value for --iterations")?;
                iterations = parse_u64(value, "--iterations")?;
            }
            "--help" | "-h" => {
                print_single_usage();
                return Ok(());
            }
            other => return Err(format!("Unknown flag for single mode: {other}")),
        }
        i += 1;
    }

    if iterations == 0 {
        return Err("Iterations must be greater than zero".into());
    }

    let start = Instant::now();
    let estimate = leibniz_pi(iterations);
    let elapsed = start.elapsed();

    report_results(
        "Single-threaded Leibniz",
        "Iterations",
        iterations,
        elapsed,
        estimate,
        "iterations/s",
    );

    Ok(())
}

fn run_monte_carlo(args: &[String]) -> Result<(), String> {
    let mut samples = DEFAULT_MONTE_CARLO_SAMPLES;
    let mut threads: Option<usize> = None;
    let mut seed: Option<u64> = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--samples" | "-s" => {
                i += 1;
                let value = args.get(i).ok_or("Missing value for --samples")?;
                samples = parse_u64(value, "--samples")?;
            }
            "--threads" | "-t" => {
                i += 1;
                let value = args.get(i).ok_or("Missing value for --threads")?;
                threads = Some(parse_usize(value, "--threads")?);
            }
            "--seed" => {
                i += 1;
                let value = args.get(i).ok_or("Missing value for --seed")?;
                seed = Some(parse_u64(value, "--seed")?);
            }
            "--help" | "-h" => {
                print_monte_usage();
                return Ok(());
            }
            other => return Err(format!("Unknown flag for monte mode: {other}")),
        }
        i += 1;
    }

    if samples == 0 {
        return Err("Samples must be greater than zero".into());
    }

    let thread_count = threads.unwrap_or_else(default_thread_count);
    if thread_count == 0 {
        return Err("Thread count must be at least 1".into());
    }

    let base_seed = seed.unwrap_or_else(random_seed);
    let (per_thread, remainder) = split_work(samples, thread_count as u64);

    let start = Instant::now();

    let mut handles = Vec::with_capacity(thread_count);
    for idx in 0..thread_count {
        let chunk = per_thread + u64::from(idx < remainder as usize);
        let seed_for_thread = base_seed ^ (0x9E37_79B9_7F4A_7C15u64.wrapping_mul(idx as u64 + 1));
        handles.push(thread::spawn(move || monte_carlo_hits(chunk, seed_for_thread)));
    }

    let total_hits: u128 = handles
        .into_iter()
        .map(|h| u128::from(h.join().unwrap_or(0)))
        .sum();

    let elapsed = start.elapsed();
    let estimate = 4.0 * (total_hits as f64) / (samples as f64);

    report_results(
        &format!("Monte Carlo ({} threads)", thread_count),
        "Samples",
        samples,
        elapsed,
        estimate,
        "samples/s",
    );

    Ok(())
}

fn leibniz_pi(iterations: u64) -> f64 {
    let mut acc = 0.0f64;
    for k in 0..iterations {
        let term = if k % 2 == 0 { 1.0 } else { -1.0 };
        acc += term / (2 * k + 1) as f64;
    }
    4.0 * acc
}

fn monte_carlo_hits(samples: u64, seed: u64) -> u64 {
    let mut rng_state = seed | 1;
    let mut hits = 0u64;
    for _ in 0..samples {
        let x = next_unit_f64(&mut rng_state);
        let y = next_unit_f64(&mut rng_state);
        if x * x + y * y <= 1.0 {
            hits += 1;
        }
    }
    hits
}

fn next_unit_f64(state: &mut u64) -> f64 {
    // Simple LCG with shuffle to 53 bits for f64 mantissa.
    *state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
    let bits = (*state >> 11) | 0x3FF0_0000_0000_0000;
    f64::from_bits(bits) - 1.0
}

fn report_results(
    mode: &str,
    work_label: &str,
    work_units: u64,
    elapsed: Duration,
    estimate: f64,
    throughput_label: &str,
) {
    let error = (estimate - std::f64::consts::PI).abs();
    let throughput = if elapsed.as_secs_f64() > 0.0 {
        work_units as f64 / elapsed.as_secs_f64()
    } else {
        0.0
    };
    println!("Mode           : {mode}");
    println!("{work_label:<15}: {}", format_number(work_units));
    println!("PI estimate    : {:.12}", estimate);
    println!("Absolute error : {:.12}", error);
    println!("Elapsed        : {:.3} s", elapsed.as_secs_f64());
    println!("Throughput     : {:.2} {throughput_label}", throughput);
}

fn split_work(total: u64, buckets: u64) -> (u64, u64) {
    (total / buckets, total % buckets)
}

fn default_thread_count() -> usize {
    thread::available_parallelism()
        .map(|v| v.get())
        .unwrap_or(1)
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64 ^ 0xA5A5_5A5A_A5A5_5A5A)
        .unwrap_or(0xDEAD_BEEF_DEAD_BEEF)
        | 1
}

fn parse_u64(value: &str, flag: &str) -> Result<u64, String> {
    value
        .replace('_', "")
        .parse::<u64>()
        .map_err(|_| format!("Could not parse value for {flag}: {value}"))
}

fn parse_usize(value: &str, flag: &str) -> Result<usize, String> {
    value
        .replace('_', "")
        .parse::<usize>()
        .map_err(|_| format!("Could not parse value for {flag}: {value}"))
}

fn format_number(value: u64) -> String {
    let mut s = value.to_string();
    let mut out = String::new();
    while s.len() > 3 {
        let rest = s.split_off(s.len() - 3);
        if out.is_empty() {
            out = rest;
        } else {
            out = format!("{rest},{out}");
        }
    }
    if out.is_empty() {
        s
    } else {
        format!("{s},{out}")
    }
}

fn matches_help(flag: &str) -> bool {
    flag == "-h" || flag == "--help" || flag == "help"
}

fn print_global_usage() {
    println!("PI Benchmark");
    println!("Usage: pi-benchmark <mode> [options]");
    println!();
    println!("Modes:");
    println!("  single        Single-threaded Leibniz series");
    println!("  monte         Multi-threaded Monte Carlo (embarrassingly parallel)");
    println!();
    println!("Run `pi-benchmark <mode> --help` for mode-specific options.");
}

fn print_mode_usage(mode: &str) {
    match mode {
        "single" | "leibniz" => print_single_usage(),
        "monte" | "monte-carlo" | "multi" | "multi-thread" => print_monte_usage(),
        _ => print_global_usage(),
    }
}

fn print_single_usage() {
    println!("Usage: pi-benchmark single [--iterations <u64>]");
    println!("  --iterations, -n   Number of Leibniz iterations (default {DEFAULT_LEIBNIZ_ITERATIONS})");
}

fn print_monte_usage() {
    println!("Usage: pi-benchmark monte [--samples <u64>] [--threads <usize>] [--seed <u64>]");
    println!("  --samples, -s   Total random points to generate (default {DEFAULT_MONTE_CARLO_SAMPLES})");
    println!("  --threads, -t   Number of worker threads (default: system parallelism)");
    println!("  --seed          Optional RNG seed for reproducibility");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leibniz_converges() {
        let estimate = leibniz_pi(5_000_000);
        assert!((estimate - std::f64::consts::PI).abs() < 1e-4);
    }

    #[test]
    fn monte_carlo_hits_respects_bounds() {
        // Use a tiny sample size to keep the test fast.
        let hits = monte_carlo_hits(1_000, 12345);
        assert!(hits <= 1_000);
    }
}
