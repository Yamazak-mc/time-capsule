# `time-capsule`
time-capsule encodes a file into a format that cannot be decoded until a specified time.

# How To Use

## Encoding
Use the following command to encode a file `FILE_PATH`, locking the file for a duration of `DURATION` time.
```
time-capsule encode <FILE_PATH> <DURATION>
```
This operation can fail under these conditions:
 - Failure to read the file to encode (`IOError`)
 - Failure to create an output file (`IOError`)
   This occurs when the file `path/to/file.your-file-ext.time-capsule` already exists.
 - The specified `DURATION` is invalid

### Duration
`parse_duration` crate is used for parsing `DURATION`.
See its [documentation]((https://docs.rs/parse_duration/latest/parse_duration/)) for more info.

## Decoding
Use the following command to try decode a time-capsule file`FILE_PATH`.
```
time-capsule decode <FILE_PATH>
```
This operation can fail under these conditions:
 - Failure to read the file to decode (`IOError`)
 - Failure to create an output file (`IOError`)
 - Wrong file format
 - The file is still locked and cannot be decoded yet