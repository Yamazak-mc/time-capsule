# `time-capsule`
time-capsule encodes a file into a format that cannot be decoded until a specified time.

# How To Use

## Help
Use the following commands for the details of the commands.
```
time-capsule --help

time-capsule <SUBCOMMAND> --help
```

## Encoding
Use the following command to encode a file `FILE_PATH`, locking the file for a duration of `DURATION` time.
If you specify `--rm`, the source file will be removed automatically only if succeeded in encoding the source file.
```
time-capsule encode -i <FILE_PATH> -d <DURATION> (--rm)
```
This operation can fail under these conditions:
 - Failure to read the file to encode (`IOError`)
 - The specified `DURATION` is invalid

### Duration
`parse_duration` crate is used for parsing `DURATION`.
See its [documentation](https://docs.rs/parse_duration/latest/parse_duration/) for more info.

## Decoding
Use the following command to try decode a time-capsule file `FILE_PATH`.
If the operation is successful, the source file will be deleted automatically. To prevent this, specify `--no-rm`.
```
time-capsule decode -i <FILE_PATH> (--no-rm)
```
This operation can fail under these conditions:
 - Failure to read the file to decode (`IOError`)
 - Failure to create an output file (`IOError`)
 - Wrong file format
 - The file is still locked and cannot be decoded yet