use super::MAGIC_NUMBERS;
use chrono::{DateTime, FixedOffset, Local};
use displaydoc::Display;
use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read},
    path::Path,
    string::FromUtf8Error,
};

// Decodes a time-capsule file.
pub fn decode(file: impl AsRef<Path>, remove_file: bool) -> Result<(), DecodeError> {
    match Decoder::new(&file)?.file_format_checked()?.decode() {
        Ok(_) => {
            if remove_file {
                fs::remove_file(file).expect("should be able to remove file");
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Decodes all files in the directory.
///
/// This function does not decode files in the sub directories.
pub fn decode_all(directory: impl AsRef<Path>, remove_files: bool) -> Result<(), DecodeError> {
    for entry in fs::read_dir(&directory)? {
        let path = entry?.path();
        if path.is_file() {
            if let Err(err) = decode(path, remove_files) {
                println!("{}", err);
            }
        }
    }
    Ok(())
}

/// Describes an error occurred while decoding a file.
#[remain::sorted]
#[derive(thiserror::Error, Debug, Display)]
pub enum DecodeError {
    /// directory is not found
    DirectoryNotFound,

    /// file is not time-capsule format, which cannot be decoded
    InvalidFileFormat,

    /// failed to I/O
    IO(#[from] io::Error),

    /// file cannot be decoded until {0}
    Locked(DateTime<FixedOffset>),

    /// failed to parse time
    Parse(#[from] chrono::ParseError),

    /// failed to convert a string from a UTF-8 bytes
    Utf8(#[from] FromUtf8Error),
}

struct Decoder<P> {
    src: P,
    r: BufReader<File>,
}

impl<P: AsRef<Path>> Decoder<P> {
    fn new(file: P) -> io::Result<Self> {
        let reader = BufReader::new(File::open(&file)?);
        Ok(Self {
            src: file,
            r: reader,
        })
    }

    fn file_format_checked(mut self) -> Result<Self, DecodeError> {
        let mut buf = [0; 8];
        self.r.read_exact(&mut buf)?;
        if buf != MAGIC_NUMBERS {
            return Err(DecodeError::InvalidFileFormat);
        }
        Ok(self)
    }

    fn decode(&mut self) -> Result<(), DecodeError> {
        let output_filename = self.read_string()?;

        // Check time
        let time = DateTime::parse_from_rfc3339(&self.read_string()?)?;
        if Local::now() < time {
            return Err(DecodeError::Locked(time));
        }

        // Write an output file
        let output_path = {
            let mut f = self.src.as_ref().to_path_buf();
            f.set_file_name(&output_filename);
            f
        };
        let mut writer = BufWriter::new(File::create_new(output_path)?);
        io::copy(&mut self.r, &mut writer)?;

        // Log
        println!(
            "decoded file successfully:\n - src: {}\n - decoded file name: {}",
            self.src.as_ref().display(),
            output_filename
        );

        Ok(())
    }

    fn read_string(&mut self) -> Result<String, DecodeError> {
        let len = {
            let mut buf = [0; 8];
            self.r.read_exact(&mut buf)?;
            u64::from_be_bytes(buf) as usize
        };
        let mut buf = vec![0; len];
        self.r.read_exact(&mut buf)?;
        Ok(String::from_utf8(buf)?)
    }
}
