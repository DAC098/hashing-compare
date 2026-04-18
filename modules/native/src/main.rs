use std::{
    cmp::Ordering,
    fs::OpenOptions,
    hint::black_box,
    io::{BufWriter, Write},
    path::PathBuf,
    slice::Chunks,
    time::{Duration, Instant, SystemTime},
};

use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use rand::{RngExt, SeedableRng, rngs::StdRng};

mod unit;

#[derive(Debug, Parser)]
struct CliArgs {
    #[command(flatten)]
    testing: TestingArgs,

    #[command(subcommand)]
    cmd: HashArg,
}

#[derive(Debug, Args)]
struct TestingArgs {
    #[arg(short, long, default_value = "50")]
    warmup: usize,

    #[arg(short, long, default_value = "100")]
    iterations: usize,

    #[arg(long)]
    output: PathBuf,

    #[arg(long, default_value = "512")]
    chunk_size: usize,
}

#[derive(Debug, Subcommand)]
enum InputArg {
    Rand {
        #[arg(short, long)]
        seed: Option<u64>,

        #[arg(long)]
        total_chunks: usize,
    },
    File {
        #[arg(long)]
        path: PathBuf,
    }
}

#[derive(Debug, Subcommand)]
enum HashArg {
    Md5 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha1 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha2_256 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha2_384 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha2_512 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha3_256 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha3_384 {
        #[command(subcommand)]
        input: InputArg
    },
    Sha3_512 {
        #[command(subcommand)]
        input: InputArg
    },
    Blake3 {
        #[command(subcommand)]
        input: InputArg
    },
}

type ArgsTuple = (&'static str, InputArg, &'static dyn Fn(Chunks<'_, u8>) -> Vec<u8>);

fn main() -> anyhow::Result<()> {
    let CliArgs {
        cmd,
        testing,
    } = CliArgs::parse();

    let (name, input, cb): ArgsTuple = match cmd {
        HashArg::Md5 { input } => ("md5", input, &run_md5),
        HashArg::Sha1 { input } => ("sha1", input, &run_sha1),
        HashArg::Sha2_256 { input } => ("sha2_256", input, &run_sha2_256),
        HashArg::Sha2_384 { input } => ("sha2_384", input, &run_sha2_384),
        HashArg::Sha2_512 { input } => ("sha2_512", input, &run_sha2_512),
        HashArg::Sha3_256 { input } => ("sha3_256", input, &run_sha3_256),
        HashArg::Sha3_384 { input } => ("sha3_384", input, &run_sha3_384),
        HashArg::Sha3_512 { input } => ("sha3_512", input, &run_sha3_512),
        HashArg::Blake3 { input } => ("blake3", input, &run_blake3),
    };

    run_hash_test(name, testing, input, cb)
}

fn run_hash_test<F, T>(
    name: &str,
    testing: TestingArgs,
    input: InputArg,
    cb: F,
) -> anyhow::Result<()>
where
    F: Fn(Chunks<'_, u8>) -> T,
{
    let mut results: Vec<f64> = Vec::with_capacity(testing.iterations);

    let output_path = get_output_path(name, &testing);
    let input_data = get_input(&testing, &input);

    let status_duration = Duration::new(10, 0);
    let test_start = Instant::now();
    let mut last_status = Instant::now();

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
    let mut output = BufWriter::new(output_file);

    writeln!(&mut output, "env,hash,bytes,chunk_size,iterations,warmup")
        .context("failed to write csv header")?;
    writeln!(
        &mut output,
        "native,{name},{},{},{},{}",
        input_data.len(), testing.chunk_size, testing.iterations, testing.warmup
    )
    .context("failed to write test info to csv")?;
    writeln!(&mut output, "times").context("failed to write results header to csv")?;

    let total = testing.iterations + testing.warmup;

    for it in 0..total {
        let chunks = input_data.as_slice().chunks(testing.chunk_size);

        let start = Instant::now();

        black_box(cb(chunks));

        let duration = start.elapsed();

        if it >= testing.warmup {
            results.push(duration.as_secs_f64());

            writeln!(&mut output, "{}", duration.as_secs_f64())
                .context("failed to write result to csv")?;
        }

        if last_status.elapsed() > status_duration {
            println!(
                "{name} {it} / {total} {:.01}% {:#?}",
                (it as f64 / total as f64) * 100.0,
                test_start.elapsed()
            );

            last_status = Instant::now();
        }
    }

    log_results(&results, input_data.len());

    Ok(())
}

use md5::{Digest, Md5};

fn run_md5<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = Md5::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha1<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha1::Sha1::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha2_256<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha2_384<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha384::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha2_512<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha2::Sha512::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha3_256<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha3::Sha3_256::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha3_384<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha3::Sha3_384::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_sha3_512<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = sha3::Sha3_512::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().to_vec()
}

fn run_blake3<'a>(chunks: Chunks<'a, u8>) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();

    for chunk in chunks {
        hasher.update(chunk);
    }

    hasher.finalize().as_bytes().to_vec()
}

fn get_time() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .expect("check clock settings as system time is before UNIX_EPOCH")
}

fn get_input(testing: &TestingArgs, input: &InputArg) -> Vec<u8> {
    match input {
        InputArg::Rand { seed, total_chunks } => {
            let seed = seed.unwrap_or(rand::random());

            println!("seed: {seed}");

            let mut rng = StdRng::seed_from_u64(seed);

            println!(
                "generating {} bytes",
                unit::FmtUnit::new((total_chunks * testing.chunk_size) as u64, "B")
            );

            let mut tmp = vec![0; total_chunks * testing.chunk_size];

            rng.fill(tmp.as_mut_slice());

            tmp
        }
        InputArg::File { path } => {
            std::fs::read(&path).expect("failed to read contents of input file")
        }
    }
}

fn get_output_path(name: &str, testing: &TestingArgs) -> PathBuf {
    if testing.output.is_dir() {
        testing
            .output
            .join(format!("native_{name}_{}.csv", get_time()))
    } else {
        testing.output.clone()
    }
}

fn log_results(results: &[f64], bytes: usize) {
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

    let mut outliers: usize = 0;

    for value in results {
        let score = ((*value - average) / std_dev).abs();

        if matches!(score.total_cmp(&3.0), Ordering::Greater) {
            outliers += 1;
        }
    }

    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(0.0);
    let hashing_speed = bytes as f64 / average;

    println!("results: ~{average:.09}+-{std_dev:0.9}");
    println!("    sem: {sem:0.12} min: {min:.09} max: {max:.09}");
    println!(
        "    outliers: {outliers} / {} {:.01}%",
        results.len(),
        (outliers as f64 / results.len() as f64) * 100.0
    );
    println!("    speed: {}/s", unit::FmtUnitF64::new(hashing_speed, "B"));
}
