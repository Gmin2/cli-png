# PNG Message CLI

A command-line tool for encoding and decoding secret messages in PNG files. This tool allows you to add, retrieve, and manage hidden text messages within PNG files without affecting their visual appearance.

## Features

- Encode secret messages into PNG files
- Decode hidden messages from PNG files
- Remove hidden messages from PNG files
- Print information about PNG chunks
- Preserves original image quality
- Support for custom chunk types

## Installation

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

Clone the repository:
```bash
git clone https://github.com/Gmin2/cli-png.git
cd png-cli
```
Build the project:
```bash
cargo build --release
```
The binary will be available at `target/release/pngme`
## Usage

Encode a Message:

Add a secret message to a PNG file:
```bash
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE> [OUTPUT_FILE]
```

Example:
```bash
pngme encode ./image.png ruSt "This is a secret message!"
```

With custom output file:
```bash
pngme encode ./image.png ruSt "This is a secret message!" -o ./output.png
```

Decode a Message:
Read a hidden message from a PNG file:

```bash
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

Example:
```bash
pngme decode ./image.png ruSt
```

Remove a Message:
Remove a hidden message from a PNG file:

```bash
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

Example:
```bash
pngme remove ./image.png ruSt
```

Print Chunk Information:
Print information about PNG chunks in a PNG file:

```bash
pngme info <FILE_PATH>
```

Example:
```bash
pngme info ./image.png
```

Test can be run with:
```
cargo test
```
 Made from this [specs](https://jrdngr.github.io/pngme_book/hints/chapter_5_hints.html) inspired from [pngme](https://jrdngr.github.io/pngme_book/setup.html)
