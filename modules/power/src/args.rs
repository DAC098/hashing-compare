use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf,
    str::FromStr,
    time::Duration,
};

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(short, long, default_value = "512")]
    pub chunk_size: ChunkSize,

    #[arg(short, long, default_value = "60", value_parser = parse_secs)]
    pub duration: Duration,

    #[arg(short, long, default_value = "250", value_parser = parse_millis)]
    pub rate: Duration,

    #[arg(long, default_value = "60", value_parser = parse_secs)]
    pub delay: Duration,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(short, long)]
    pub quiet: bool,

    #[arg(long)]
    pub include_time: bool,

    pub exe: ExeOpt,

    pub algo: AlgoOpt,

    pub input: PathBuf,
}

#[derive(Debug, Clone)]
pub enum ChunkSize {
    All,
    Known(u64),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ExeOpt {
    Native,
    Wasm,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AlgoOpt {
    All,
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

impl FromStr for ChunkSize {
    type Err = &'static str;

    fn from_str(given: &str) -> Result<Self, Self::Err> {
        if given == "all" {
            Ok(Self::All)
        } else {
            let Ok(parsed) = u64::from_str(given) else {
                return Err("invalid chunk-size provided");
            };

            Ok(Self::Known(parsed))
        }
    }
}

impl ExeOpt {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Wasm => "wasm",
        }
    }
}

impl Display for ExeOpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.as_str().fmt(f)
    }
}

impl AlgoOpt {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Md5 => "md5",
            Self::Sha1 => "sha1",
            Self::Sha2_256 => "sha2-256",
            Self::Sha2_384 => "sha2-384",
            Self::Sha2_512 => "sha2-512",
            Self::Sha3_256 => "sha3-256",
            Self::Sha3_384 => "sha3-384",
            Self::Sha3_512 => "sha3-512",
            Self::Blake3 => "blake3",
        }
    }
}

impl Display for AlgoOpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.as_str().fmt(f)
    }
}

impl AsRef<str> for AlgoOpt {
    fn as_ref(&self) -> &'static str {
        self.as_str()
    }
}

fn parse_secs(given: &str) -> Result<Duration, &'static str> {
    let Ok(secs) = u64::from_str(given) else {
        return Err("invalid seconds value provided");
    };

    Ok(Duration::new(secs, 0))
}

fn parse_millis(given: &str) -> Result<Duration, &'static str> {
    let Ok(millis) = u64::from_str(given) else {
        return Err("invalid milliseconds value provided");
    };

    Ok(Duration::from_millis(millis))
}
