use super::MAGIC_NUMBERS;
use chrono::{DateTime, Duration, Local};
use displaydoc::Display;
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Write},
    path::Path,
};

/// Encodes a file to a time-capsule format.
pub fn encode(
    file: impl AsRef<Path>,
    duration: &str,
    remove_file: bool,
) -> Result<(), EncodeError> {
    let time = duration_str_to_time(duration)?;
    match encode_impl(&file, &time) {
        Ok(_) => {
            if remove_file {
                fs::remove_file(file).expect("should be able to remove file");
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Encodes all files in the directory.
///
/// This function does not encode files in the sub directories.
pub fn encode_all(
    directory: impl AsRef<Path>,
    duration: &str,
    remove_files: bool,
) -> Result<(), EncodeError> {
    let time = duration_str_to_time(duration)?;

    for entry in fs::read_dir(&directory)? {
        let path = entry?.path();
        if path.is_file() {
            match encode_impl(&path, &time) {
                Ok(_) if remove_files => {
                    fs::remove_file(path).expect("should be able to remove file");
                }
                Err(err) => println!("{}", err),
                _ => (),
            }
        }
    }
    Ok(())
}

fn encode_impl(file: impl AsRef<Path>, time: &DateTime<Local>) -> Result<(), EncodeError> {
    let mut encoder = Encoder::new(file)?;
    encoder.encode(time)?;

    Ok(())
}

/// Describes an error occurred while encoding a file.
#[remain::sorted]
#[derive(thiserror::Error, Debug, Display)]
pub enum EncodeError {
    /// file is time-capsule format, which cannot be encoded
    AlreadyEncoded,

    /// duration range is invalid
    InvalidDuration(#[from] chrono::OutOfRangeError),

    /// failed to add duration to the current time
    InvalidTime,

    /// failed to I/O
    IO(#[from] io::Error),

    /// failed to parse duration
    ParseDuration(#[from] parse_duration::parse::Error),
}

pub fn duration_str_to_time(duration: &str) -> Result<DateTime<Local>, EncodeError> {
    let duration = Duration::from_std(parse_duration::parse(duration)?)?;

    let Some(time) = Local::now().checked_add_signed(duration) else {
        return Err(EncodeError::InvalidTime);
    };

    Ok(time)
}

struct Encoder<P> {
    src: P,
    filename: String,
    r: BufReader<File>,
}

impl<P: AsRef<Path>> Encoder<P> {
    fn new(file: P) -> io::Result<Self> {
        let filename = file
            .as_ref()
            .file_name()
            .expect("file name should exist")
            .to_string_lossy()
            .into();

        let r = BufReader::new(File::open(&file)?);

        Ok(Self {
            src: file,
            filename,
            r,
        })
    }

    fn encode(&mut self, time: &DateTime<Local>) -> Result<(), EncodeError> {
        let output_filename = format!(".time-capsule-{}", uuid::Uuid::new_v4());
        let output_path = {
            let mut f = self.src.as_ref().to_path_buf();
            f.set_file_name(&output_filename);
            f
        };
        let mut writer = BufWriter::new(File::create_new(&output_path)?);

        // Write a header
        writer.write_all(&MAGIC_NUMBERS)?;
        writer.write_all(&(self.filename.len() as u64).to_be_bytes())?;
        writer.write_all(self.filename.as_bytes())?;

        // Write time
        let serialized = time.to_rfc3339();
        writer.write_all(&(serialized.len() as u64).to_be_bytes())?;
        writer.write_all(serialized.as_bytes())?;

        // Copy the content of source file.
        io::copy(&mut self.r, &mut writer)?;

        // Log
        println!(
            "encoded file successfully:\n - src: {}\n - encoded file name: {}",
            self.src.as_ref().display(),
            output_filename
        );

        Ok(())
    }
}
