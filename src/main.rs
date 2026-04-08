use std::{
    cmp::Ordering,
    fs::OpenOptions,
    path::PathBuf,
    slice::Chunks,
    time::{Duration, Instant, SystemTime},
};

use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use rand::{RngExt, SeedableRng, rngs::StdRng};

#[derive(Debug, Parser)]
struct CliArgs {
    #[command(flatten)]
    testing: TestingArgs,

    #[command(flatten)]
    input: InputArg,

    #[command(subcommand)]
    cmd: HashArg,
}

#[derive(Debug, Args)]
struct TestingArgs {
    #[arg(short, long, default_value = "100")]
    warmup: usize,

    #[arg(short, long, default_value = "500")]
    iterations: usize,

    #[arg(long)]
    output: PathBuf,

    #[arg(long)]
    chunk_size: usize,
}

#[derive(Debug, Args)]
struct InputArg {
    #[arg(short, long)]
    seed: Option<u64>,

    #[arg(long)]
    total_chunks: usize,
}

#[derive(Debug, Subcommand)]
enum HashArg {
    Md5,
    Sha1,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake3,
}

type NameCbTuple = (&'static str, &'static dyn Fn(Chunks<'_, u8>));

fn main() -> anyhow::Result<()> {
    let CliArgs {
        cmd,
        testing,
        input,
    } = CliArgs::parse();

    let seed = input.seed.unwrap_or(rand::random());

    println!("seed: {seed}");

    let mut rng = StdRng::seed_from_u64(seed);

    let (name, cb): NameCbTuple = match cmd {
        HashArg::Md5 => ("md5", &run_md5),
        HashArg::Sha1 => ("sha1", &run_sha1),
        HashArg::Sha2_256 => ("sha2_256", &run_sha2_256),
        HashArg::Sha2_384 => ("sha2_384", &run_sha2_384),
        HashArg::Sha2_512 => ("sha2_512", &run_sha2_512),
        HashArg::Sha3_256 => ("sha3_256", &run_sha3_256),
        HashArg::Sha3_384 => ("sha3_384", &run_sha3_384),
        HashArg::Sha3_512 => ("sha3_512", &run_sha3_512),
        HashArg::Blake3 => ("blake3", &run_blake3),
    };

    run_hash_test(name, &mut rng, testing, input, cb)
}

fn run_hash_test<F>(
    name: &str,
    rng: &mut StdRng,
    testing: TestingArgs,
    input: InputArg,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(Chunks<'_, u8>),
{
    let mut results: Vec<f64> = Vec::with_capacity(testing.iterations);

    let output_path = get_output_path(name, &testing);
    let input_data = get_input(rng, &testing, &input);

    let status_duration = Duration::new(10, 0);
    let test_start = Instant::now();
    let mut last_status = Instant::now();

    for it in 0..(testing.iterations + testing.warmup) {
        let chunks = input_data.as_slice().chunks(testing.chunk_size);

        let start = Instant::now();

        cb(chunks);

        let duration = start.elapsed();

        if it >= testing.warmup {
            results.push(duration.as_secs_f64());
        }

        if last_status.elapsed() > status_duration {
            println!("{name} {it} {:#?}", test_start.elapsed());

            last_status = Instant::now();
        }
    }

    log_results(&results);

    let result_struct = serde_json::json!({
        "env": "native",
        "hash": name,
        "chunk_size": testing.chunk_size,
        "iterations": testing.iterations,
        "warmup": testing.warmup,
        "results": results,
    });

    let output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_path)
        .with_context(|| {
            format!(
                "failed to open output file for results: {}",
                output_path.display()
            )
        })?;

    serde_json::to_writer(output_file, &result_struct)
        .context("failed to write results to output file")?;

    Ok(())
}

use md5::{Digest, Md5};

fn run_md5<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = Md5::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha1<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha1::Sha1::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha2_256<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha2::Sha256::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha2_384<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha2::Sha384::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha2_512<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha2::Sha512::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha3_256<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha3::Sha3_256::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha3_384<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha3::Sha3_384::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_sha3_512<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = sha3::Sha3_512::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn run_blake3<'a>(chunks: Chunks<'a, u8>) {
    let mut hasher = blake3::Hasher::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    let _ = hasher.finalize();
}

fn get_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .expect("check clock settings as system time is before UNIX_EPOCH")
}

fn get_input(rng: &mut StdRng, testing: &TestingArgs, input: &InputArg) -> Vec<u8> {
    println!(
        "generating {} bytes",
        input.total_chunks * testing.chunk_size
    );

    let mut tmp = Vec::with_capacity(input.total_chunks * testing.chunk_size);

    rng.fill(tmp.as_mut_slice());

    tmp
}

fn get_output_path(name: &str, testing: &TestingArgs) -> PathBuf {
    if testing.output.is_dir() {
        testing
            .output
            .join(format!("native_{name}_{}.json", get_time()))
    } else {
        testing.output.clone()
    }
}

fn log_results(results: &[f64]) {
    let mut total_time = 0.0f64;
    let mut min = None::<f64>;
    let mut max = None::<f64>;

    for value in results {
        total_time += *value;

        min = if let Some(v) = min {
            Some(match v.total_cmp(value) {
                Ordering::Less => v,
                _ => *value,
            })
        } else {
            Some(*value)
        };

        max = if let Some(v) = max {
            Some(match v.total_cmp(value) {
                Ordering::Greater => v,
                _ => *value,
            })
        } else {
            Some(*value)
        };
    }

    let average = total_time / results.len() as f64;
    let (std_dev, sem) = {
        let mut variance = 0.0f64;

        for value in results {
            variance += (*value - average).powf(2.0);
        }

        variance /= results.len() as f64;

        let sd = variance.sqrt();

        (sd, sd / results.len() as f64)
    };

    let mut outliers = 0;

    for value in results {
        let score = ((*value - average) / std_dev).abs();

        if matches!(score.total_cmp(&3.0), Ordering::Greater) {
            outliers += 1;
        }
    }

    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(0.0);

    println!("results: ~{average:.09}+-{std_dev:0.9}");
    println!("    sem: {sem:0.12} min: {min:.09} max: {max:.09}");
    println!("    outliers: {outliers}");
}
