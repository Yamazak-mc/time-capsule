use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

use time_capsule::{decode, decode_all, encode, encode::duration_str_to_time, encode_all};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[remain::sorted]
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Decodes a time-capsule file
    Decode { file: PathBuf },

    /// Decodes all time-capsule files in the directory
    DecodeAll { directory: PathBuf },

    /// Encodes a file to time-capsule format
    Encode { file: PathBuf, duration: String },

    /// Encodes all files in the directory
    EncodeAll {
        directory: PathBuf,
        duration: String,
    },

    /// Tests a duration string input
    TestDuration { duration: String },
}

impl Commands {
    /// Executes the comand.
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Encode { file, duration } => encode(file, duration)?,
            Self::EncodeAll {
                directory,
                duration,
            } => encode_all(directory, duration)?,
            Self::Decode { file } => decode(file)?,
            Self::DecodeAll { directory } => decode_all(directory)?,
            Self::TestDuration { duration } => {
                println!("Input    : {}", duration);
                println!("Timestamp: {}", duration_str_to_time(duration)?);
            }
        };
        Ok(())
    }
}
