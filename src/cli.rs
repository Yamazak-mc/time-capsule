use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

use time_capsule::{decode, decode_all, encode, encode::duration_str_to_time, encode_all};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[remain::sorted]
#[derive(Subcommand, Debug)]
#[clap(about, long_about = None)]
pub enum Commands {
    /// Decode a time-capsule file
    Decode {
        /// Input file path
        #[arg(short)]
        input: PathBuf,

        /// Keep source file after decoding
        #[arg(long)]
        no_rm: bool,
    },

    /// Decode all time-capsule files in the directory
    DecodeAll {
        /// Input directory
        #[arg(short)]
        input: PathBuf,

        /// Keep source files after decoding
        #[arg(long)]
        no_rm: bool,
    },

    /// Encode a file to time-capsule format
    Encode {
        /// Input file
        #[arg(short)]
        input: PathBuf,

        /// Duration of file locking
        #[arg(short)]
        duration: String,

        /// Remove source file after encoding
        #[arg(long)]
        rm: bool,
    },

    /// Encode all files in the directory
    EncodeAll {
        /// Input directory
        #[arg(short)]
        input: PathBuf,

        /// Duration of file locking
        #[arg(short)]
        duration: String,

        /// Remove source files after encoding
        #[arg(long)]
        rm: bool,
    },

    /// Test a duration string input
    TestDuration { duration: String },
}

impl Commands {
    /// Executes the comand.
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Self::Encode {
                input: i,
                duration,
                rm,
            } => {
                encode(i, duration, *rm)?;
            }
            Self::EncodeAll {
                input: i,
                duration,
                rm,
            } => {
                encode_all(i, duration, *rm)?;
            }
            Self::Decode { input: i, no_rm } => {
                decode(i, !no_rm)?;
            }
            Self::DecodeAll { input: i, no_rm } => {
                decode_all(i, !no_rm)?;
            }
            Self::TestDuration { duration } => {
                println!("Input    : {}", duration);
                println!("Timestamp: {}", duration_str_to_time(duration)?);
            }
        };
        Ok(())
    }
}
