use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::Context;

pub struct Output(Option<BufWriter<File>>);

use crate::args::TestingArgs;
use crate::time::get_time;

impl Output {
    pub fn new(name: &str, testing: &TestingArgs) -> anyhow::Result<Self> {
        if let Some(output_path) = get_output_path(name, &testing) {
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

            Ok(Self(Some(BufWriter::new(output_file))))
        } else {
            Ok(Self(None))
        }
    }

    pub fn enabled(&self) -> bool {
        self.0.is_some()
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        if let Some(inner) = &mut self.0 {
            inner.write(buf)
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        if let Some(inner) = &mut self.0 {
            inner.flush()
        } else {
            Ok(())
        }
    }
}

fn get_output_path(name: &str, testing: &TestingArgs) -> Option<PathBuf> {
    let output = testing.output.as_ref()?;

    if output.is_dir() {
        let time = get_time();
        let mut count: usize = 1;

        loop {
            let name = format!("native_{name}_{time}_{count}.csv");
            let check = output.join(&name);

            if !check.exists() {
                return Some(check);
            }

            count += 1;
        }
    } else {
        Some(output.clone())
    }
}
