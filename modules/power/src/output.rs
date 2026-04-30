use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::Path,
};

pub struct Output(Option<BufWriter<File>>);

impl Output {
    pub fn new<P>(path: Option<P>) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        if let Some(output_path) = path {
            let output_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&output_path)?;

            Ok(Self(Some(BufWriter::new(output_file))))
        } else {
            Ok(Self(None))
        }
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
