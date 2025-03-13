# Mixtape Engine

Mixtape Engine is a high-performance, multithreaded media conversion tool built with Rust and powered by FFmpeg. It supports audio, video, and image format conversion, offering efficient parallel processing, batch file handling, and real-time progress tracking.

## Features

- Batch processing for multiple file conversions
- Multithreaded execution utilizing all CPU cores
- FFmpeg integration supporting various media formats
- Dry-run mode to simulate conversions without writing output
- Custom FFmpeg options for advanced configuration
- Detailed error and success logging
- Real-time progress tracking with estimated completion time

## Requirements

- Rust (latest stable version) – Install via [Rustup](https://rustup.rs)
- FFmpeg – Install using:
  - macOS: `brew install ffmpeg`
  - Linux: `sudo apt install ffmpeg`
  - Windows: `choco install ffmpeg`

Mixtape can also bundle FFmpeg internally, eliminating the need for manual installation.

## Installation

Clone the repository:

```sh
git clone https://github.com/mixtape-app/mixtape-engine.git
cd mixtape-engine
```

Build the project:

```sh
cargo build --release
```

## Usage

### Basic Conversion

Convert a single file:

```sh
mixtape input.mp4 output.mp3
```

This extracts the audio from `input.mp4` and saves it as `output.mp3`.

### Batch Conversion

Convert multiple files in a single command:

```sh
mixtape input1.mp4 output1.mp3 input2.avi output2.mp4
```

Each input file must be paired with a corresponding output file.

### Force Overwrite

To overwrite existing output files without confirmation:

```sh
mixtape input.mp4 output.mp3 --force
```

### Dry-Run Mode

Simulate a conversion without actually generating an output file:

```sh
mixtape input.mp4 output.mp3 --dry-run
```

This verifies if the command will work as expected.

### Custom FFmpeg Options

Pass additional FFmpeg parameters for more control:

```sh
mixtape input.mp4 output.mp3 --options "-b:a 192k -ac 2"
```

Example options:

- `-b:a 192k` – Set audio bitrate to 192kbps
- `-ac 2` – Force stereo output

### Checking FFmpeg Availability

Run the following command to check if FFmpeg is installed:

```sh
mixtape --check
```

If FFmpeg is missing, Mixtape will display an appropriate warning message.

## Logging

Mixtape logs all errors and successful conversions in `mixtape.log`.

View logs:

```sh
cat mixtape.log
```

Example log entries:

```
[2025-03-13 14:32:10] ERROR: Input file 'missing.mp4' does not exist.
[2025-03-13 14:35:21] SUCCESS: Successfully converted 'input.mp4' to 'output.mp3'.
```

## Running Tests

To verify functionality, run:

```sh
cargo test
```

## Configuration & Customization

Mixtape Engine is designed for flexibility. You can modify:

- Default logging path (`logging.rs`)
- Supported formats (`utils.rs`)
- Thread utilization for CPU optimization (`conversion.rs`)
- Progress tracking display (`indicatif` integration)

## Project Structure

```
mixtape-engine/
│── src/
│   ├── main.rs        # Entry point
│   ├── lib.rs         # Module entry
│   ├── cli.rs         # CLI argument handling
│   ├── utils.rs       # Helper functions
│   ├── logging.rs     # Logging functions
│   ├── conversion.rs  # FFmpeg integration
│── tests/
│   ├── cli_tests.rs
│   ├── utils_tests.rs
│   ├── logging_tests.rs
│── Cargo.toml
```

## Future Enhancements

- Graphical User Interface (GUI) with Tauri
- Web API mode to support remote processing
- Additional media format support
- Preset configurations for common conversion tasks

## Contributing

Contributions are welcome. To contribute:

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

Mixtape Engine is open-source under the MIT License.
