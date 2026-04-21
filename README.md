# Multithreaded Downloader (Rust)

A command-line file downloader built in Rust that uses parallel chunk downloads via HTTP Range requests for faster and more reliable downloads.

## Features

- Multithreaded chunk-based downloading
- Configurable thread count (1–16)
- Retry failed chunk requests
- Real-time progress tracking
- Safe merge handling (aborts if any chunk fails)
- Temporary file cleanup
- Guided output filename / extension selection

## How It Works

1. Fetches file size from server
2. Splits file into chunks
3. Downloads chunks in parallel threads
4. Tracks completed chunks with shared progress counter
5. Merges chunk files into final output
6. Cleans temporary files

## Tech Used

- Rust
- reqwest
- std::thread
- Arc + Mutex

## Run

## Example Use Cases
- Download images
- Download documents
- Learn concurrency in Rust
- Practice systems programming

## Future Improvements
- Byte-level progress bar
- Resume interrupted downloads
- Auto-detect file extension
- Better error reporting

```bash
cargo run

