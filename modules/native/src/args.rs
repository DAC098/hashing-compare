use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[command(flatten)]
    pub testing: TestingArgs,

    #[command(subcommand)]
    pub cmd: HashArg,
}

#[derive(Debug, Args)]
pub struct TestingArgs {
    #[arg(short, long, default_value = "50")]
    pub warmup: usize,

    #[arg(short, long, default_value = "100")]
    pub iterations: usize,

    #[arg(long)]
    pub output: Option<PathBuf>,

    #[arg(long, default_value = "512")]
    pub chunk_size: usize,

    #[arg(short, long)]
    pub busy: Option<u64>,

    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Debug, Subcommand)]
pub enum InputArg {
    Rand {
        #[arg(short, long)]
        seed: Option<u64>,

        #[arg(long)]
        total_chunks: usize,
    },
    File {
        #[arg(long)]
        path: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
pub enum HashArg {
    Md5 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha1 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha2_256 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha2_384 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha2_512 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha3_256 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha3_384 {
        #[command(subcommand)]
        input: InputArg,
    },
    Sha3_512 {
        #[command(subcommand)]
        input: InputArg,
    },
    Blake3 {
        #[command(subcommand)]
        input: InputArg,
    },
}
