# regions-cli

## About the Project

`regions-cli` is a command line tool developed to concatenate multiple files into one, and in turn, extract these files from the concatenated file. The motivation behind this project was to make handling multiple files easier when working with OpenAI's GPT-4, allowing the model to efficiently perform modifications over multiple files.

## Prerequisites

To use `regions-cli` you'll need to have Rust installed. You can obtain it from the [official Rust site](https://www.rust-lang.org/tools/install).

## Installation

Follow these steps to install `regions-cli` on your system:

1. Clone the `regions-cli` repository.

   ```bash
   git clone https://github.com/yourgithubusername/regions-cli.git
   ```

2. Enter the project directory and compile the code.

   ```bash
   cd regions-cli
   cargo build --release
   ```

3. Move the resulting binary to a folder in your PATH.
   ```bash
   mv target/release/regions /usr/local/bin
   ```

Now you can use `regions-cli` from anywhere in your terminal.

## Usage

The `regions-cli` project provides two main subcommands: `concatenate` and `extract`.

### `concatenate` Command

This command concatenates the contents of several files into a single one. You can use it as follows:

```bash
regions concatenate --output output_file [input_files]
```

For example:

```bash
regions concatenate --output result.txt input1.txt input2.txt input3.txt
```

This will concatenate the contents of `input1.txt`, `input2.txt`, and `input3.txt` into `result.txt`, delimiting each file's contents with `//#region {file_name}` and `//#endregion`.

### `extract` Command

This command performs the inverse operation of `concatenate`. It extracts the contents of a `concatenate` output file and creates the original files. Use the command like this:

```bash
regions extract output_file
```

For example:

```bash
regions extract result.txt
```

This will create the files `input1.txt`, `input2.txt`, and `input3.txt` from `result.txt`.

## License

This project is under the MIT license. For more information, please refer to the `LICENSE` file.
