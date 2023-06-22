use clap::{Parser, Subcommand};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    subcommand: MySub,
}

#[derive(Subcommand)]
enum MySub {
    #[clap(name = "concatenate")]
    Concatenate(ConcatenateCommand),

    #[clap(name = "extract")]
    Extract(ExtractCommand),
}

#[derive(Parser)]
struct ConcatenateCommand {
    /// Paths to the files or directories to concatenate
    #[clap(name = "PATHS", required = true)]
    paths: Vec<PathBuf>,

    /// Output file
    #[clap(short, long)]
    output: PathBuf,
}

#[derive(Parser)]
struct ExtractCommand {
    /// Input file containing regions
    #[clap(name = "INPUT")]
    input: PathBuf,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.subcommand {
        MySub::Concatenate(command) => concatenate(command.paths, command.output)?,
        MySub::Extract(command) => extract(command.input)?,
    }

    Ok(())
}

fn concatenate(paths: Vec<PathBuf>, output: PathBuf) -> io::Result<()> {
    // Create the output file
    let mut output_file = File::create(&output)?;

    // Loop through each provided path
    for path in paths {
        // Check if the path is a directory or a file
        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            // If it's a directory, loop through all the files in it
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.metadata()?.is_file() {
                    // If it's a file, read its content and write it to the output file
                    let mut file = File::open(entry.path())?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)?;
                    writeln!(&mut output_file, "//#region {}", entry.path().display())?;
                    writeln!(&mut output_file, "{}", contents)?;
                    writeln!(&mut output_file, "//#endregion")?;
                }
            }
        } else if metadata.is_file() {
            // If it's a file, read its content and write it to the output file
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            writeln!(&mut output_file, "//#region {}", path.display())?;
            writeln!(&mut output_file, "{}", contents)?;
            writeln!(&mut output_file, "//#endregion")?;
        }
    }

    Ok(())
}

fn extract(input: PathBuf) -> io::Result<()> {
    // Open the input file
    let input_file = File::open(&input)?;
    let reader = BufReader::new(input_file);

    // Initialize variables to track the current file and its name
    let mut current_file: Option<BufWriter<File>> = None;

    // Loop through each line in the input file
    for line in reader.lines() {
        let line = line?;
        // Detect the start of a region
        let region_regex = Regex::new(r"^\s*//\s*#region (.*)$").unwrap();
        let endregion_regex = Regex::new(r"^\s*//\s*#endregion$").unwrap();
        if region_regex.is_match(&line) {
            let current_filename = line.trim_end().split(" ").last().unwrap();
            let path = PathBuf::from(&current_filename);

            // Create directories if they don't exist
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }

            // Create a new file and start writing to it
            current_file = Some(BufWriter::new(File::create(path)?));
        } else if endregion_regex.is_match(&line) {
            // Detect the end of a region
            // Stop writing to the current file
            if let Some(mut file) = current_file.take() {
                file.flush()?;
            }
        } else {
            // Write the current line to the current file, if there is one
            if let Some(file) = &mut current_file {
                writeln!(file, "{}", line)?;
            }
        }
    }

    Ok(())
}
