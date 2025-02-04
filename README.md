# Translate CLI

**🌐 Read this in [简体中文](README-zh-CN.md).**

## Overview

Translate CLI is a command-line tool that allows users to translate text using the Google Cloud Translation API. Users can input text via standard input, and the tool will return the translated text in the specified target language.

## Features

- Supports multiple target languages (default: Simplified Chinese `zh-CN`).
- Reads input text from standard input.
- Fetches translations using Google Cloud Translation API.
- Loads API key from a configuration file.
- Uses `.env` for environment variable support.

## Prerequisites

- Rust (with `cargo` package manager)
- A Google Cloud API key with access to the Translation API
- `.env` file or configuration file specifying the API key

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/LogicalMonster/translate-cli.git
   cd translate-cli
   ```

2. Install dependencies:
   ```sh
   cargo build --release
   ```

3. Move the binary to a directory in your `$PATH` (optional):
   ```sh
   mv target/release/translate-cli /usr/local/bin/
   ```

## Configuration

### API Key Setup
You need to provide a Google Cloud API key. There are two ways to do this:

1. **Using a Configuration File**  
   Create a configuration file (default: `~/.translate_cli_config`) with the following content:
   ```toml
   google_cloud_api_key = "your-api-key-here"
   ```
   You can also set a custom path using the `TRANSLATE_CLI_CONFIG` environment variable.

2. **Using `.env` File**  
   Create a `.env` file in the project root with the following content:
   ```sh
   GOOGLE_CLOUD_API_KEY=your-api-key-here
   ```

## Usage

1. Run the program with an optional language code:
   ```sh
   echo "Hello, world!" | translate-cli --target=es
   ```
   Output:
   ```sh
   "Hola, mundo!"
   ```

2. If no target language is provided, it defaults to Simplified Chinese (`zh-CN`):
   ```sh
   echo "Hello, world!" | translate-cli
   ```
   Output:
   ```sh
   "你好，世界！"
   ```

## Command-Line Arguments

| Flag         | Description                          | Default  |
|-------------|----------------------------------|----------|
| `-t, --target` | Target language code (e.g., `es`, `ja`) | `zh-CN`  |

## Dependencies

- `reqwest` (HTTP client)
- `serde` and `serde_json` (JSON parsing)
- `tokio` (asynchronous runtime)
- `config` (configuration management)
- `clap` (command-line argument parsing)
- `dotenv` (environment variable management)
- `shellexpand` (expands `~` in paths)

## License

This project is licensed under the GPL-3.0 License.

## Contributing

Feel free to submit issues or pull requests on GitHub!

## Author

- [GitHub](https://github.com/LogicalMonster)
