// this is going to be specific to a jetson orin nano

use std::{
    convert::AsRef,
    ffi::OsStr,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command},
    str::{FromStr, from_utf8},
    time::{Duration, Instant, SystemTime},
};

use clap::Parser;

mod args;
mod output;

use args::{AlgoOpt, ChunkSize, CliArgs, ExeOpt};
use output::Output;

const VOLT_INPUT: &str = "/sys/bus/i2c/drivers/ina3221/1-0040/hwmon/hwmon1/in1_input";
const AMP_INPUT: &str = "/sys/bus/i2c/drivers/ina3221/1-0040/hwmon/hwmon1/curr1_input";

const HASHES: [AlgoOpt; 9] = [
    AlgoOpt::Md5,
    AlgoOpt::Sha1,
    AlgoOpt::Sha2_256,
    AlgoOpt::Sha2_384,
    AlgoOpt::Sha2_512,
    AlgoOpt::Sha3_256,
    AlgoOpt::Sha3_384,
    AlgoOpt::Sha3_512,
    AlgoOpt::Blake3,
];
const CHUNK_SIZES: [u64; 9] = [64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

struct PowerSample {
    time: Option<f64>,
    volts: u64,
    amps: u64,
    watts: u64,
}

fn main() {
    let args = CliArgs::parse();

    match args.chunk_size {
        ChunkSize::All => {
            let mut first_chunk = true;

            for size in CHUNK_SIZES {
                if !first_chunk {
                    if !args.quiet {
                        println!("idling for {:#?}", args.delay);
                    }

                    std::thread::sleep(args.delay);
                } else {
                    first_chunk = false;
                }

                run_chunk_size(size, &args);
            }
        }
        ChunkSize::Known(size) => {
            run_chunk_size(size, &args);
        }
    }
}

impl PowerSample {
    fn collect(include_time: bool) -> Self {
        let time = if include_time {
            Some(get_secs_f64())
        } else {
            None
        };
        let volts = get_hwmon(VOLT_INPUT).unwrap_or(0);
        let amps = get_hwmon(AMP_INPUT).unwrap_or(0);
        let watts = volts * amps;

        Self {
            time,
            volts,
            amps,
            watts,
        }
    }
}

impl Display for PowerSample {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(time) = &self.time {
            write!(
                f,
                "{time:.03} v: {} a: {} w: {}",
                self.volts, self.amps, self.watts
            )
        } else {
            write!(f, "v: {} a: {} w: {}", self.volts, self.amps, self.watts)
        }
    }
}

fn run_chunk_size(size: u64, args: &CliArgs) {
    if matches!(args.algo, AlgoOpt::All) {
        let mut first_algo = true;

        for algo in HASHES {
            if !first_algo {
                if !args.quiet {
                    println!("idling for {:#?}", args.delay);
                }

                std::thread::sleep(args.delay);
            } else {
                first_algo = false;
            }

            run_algo(&algo, size, args);
        }
    } else {
        run_algo(&args.algo, size, args);
    }
}

fn run_algo(algo: &AlgoOpt, chunk_size: u64, args: &CliArgs) {
    let mut output = if let Some(output) = &args.output {
        if output.is_dir() {
            let output_name = format!("power_{}_{algo}_{chunk_size}.csv", args.exe);

            Output::new(Some(output.join(output_name))).expect("failed to open output file")
        } else {
            Output::new(Some(output.clone())).expect("failed to open output file")
        }
    } else {
        Output::new(None::<PathBuf>).unwrap()
    };

    writeln!(&mut output, "type,time,milli_volts,milli_amps")
        .expect("failed to write csv header to output");

    println!("running {algo} {chunk_size}");

    if !args.quiet {
        println!("collecting idle data");
    }

    let idle = collect_idle(args.duration, args.rate, args.include_time, args.quiet);

    if !args.quiet {
        println!("collecting process data");
    }

    let num_samples = args.duration.as_millis() / args.rate.as_millis();
    let mut collected = Vec::with_capacity(num_samples as usize + 10);
    let mut child = match &args.exe {
        ExeOpt::Native => spawn_native(args.duration.as_secs(), algo, chunk_size, &args.input),
        ExeOpt::Wasm => spawn_wasm(args.duration.as_secs(), algo, chunk_size, &args.input),
    };

    let status = loop {
        if let Some(status) = child.try_wait().expect("error when getting child status") {
            break status;
        } else {
            let sample = PowerSample::collect(args.include_time);

            if !args.quiet {
                println!("{sample}");
            }

            collected.push(sample);

            std::thread::sleep(args.rate);
        }
    };

    if !status.success() {
        println!("error code from child process: {:#?}", status.code());
    }

    for sample in idle {
        writeln!(output, "{algo},idle,{},{}", sample.volts, sample.amps)
            .expect("failed to write power sample to csv");
    }

    for sample in collected {
        writeln!(output, "{algo},run,{},{}", sample.volts, sample.amps)
            .expect("failed to write power samle to csv");
    }
}

fn collect_idle(
    duration: Duration,
    delay: Duration,
    include_time: bool,
    quiet: bool,
) -> Vec<PowerSample> {
    let num_samples = duration.as_millis() / delay.as_millis();
    let mut collected = Vec::with_capacity(num_samples as usize + 10);
    let start = Instant::now();

    while start.elapsed() < duration {
        let sample = PowerSample::collect(include_time);

        if !quiet {
            println!("{sample}");
        }

        collected.push(sample);

        std::thread::sleep(delay);
    }

    collected
}

fn spawn_native<A, P>(secs: u64, algo: A, size: u64, input_path: P) -> Child
where
    A: AsRef<str>,
    P: AsRef<OsStr>,
{
    let s_str: String = size.to_string();
    let b_str: String = secs.to_string();

    Command::new("./target/release/native")
        .args([
            "-q",
            "-b",
            &b_str,
            "--chunk-size",
            &s_str,
            algo.as_ref(),
            "file",
            "--path",
        ])
        .arg(input_path)
        .spawn()
        .expect("failed to start native program")
}

fn spawn_wasm<A, P>(secs: u64, algo: A, size: u64, input_path: P) -> Child
where
    A: AsRef<str>,
    P: AsRef<OsStr>,
{
    let s_str = size.to_string();
    let b_str = secs.to_string();

    Command::new("node")
        .args([
            "./modules/node/main.js",
            "--quiet",
            "--busy",
            &b_str,
            "--chunk-size",
            &s_str,
            algo.as_ref(),
        ])
        .arg(input_path)
        .spawn()
        .expect("failed to start wasm program")
}

fn get_hwmon<T, P>(path: P) -> Option<T>
where
    P: AsRef<Path>,
    T: FromStr,
{
    let data = std::fs::read(path).ok()?;
    let utf8 = from_utf8(&data).ok()?;

    FromStr::from_str(utf8.trim()).ok()
}

fn get_secs_f64() -> f64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .expect("check clock settings as system time is before UNIX_EPOCH")
}
