//! # time-capsule format
//!
//! time-capsule format has header bytes.
//!
//! The layout of the file will look like:
//!
//! |Name|Length(bytes)|Description|
//! |----|:-----------:|-----------|
//! | -  | 8 | magic number
//! | N  | 8 | length of the original file name
//! | -  | N | original file name
//! | M  | 8 | length of the timestamp string
//! | -  | M | timestamp string (RFC 3339)
//! | -  | until EOF | file content

const MAGIC_NUMBERS: [u8; 8] = [134, 225, 114, 190, 242, 218, 25, 79];

pub mod decode;
pub use decode::*;

pub mod encode;
pub use encode::*;
