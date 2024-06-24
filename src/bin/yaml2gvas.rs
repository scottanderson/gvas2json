#![warn(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use anyhow::Result;
use clap::Parser;
use gvas::GvasFile;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Input file to read from. Reads from stdin if not present.
    #[clap(value_parser, id = "INPUT_FILE")]
    input: Option<String>,

    /// Output file to write to. Writes to stdout if not present.
    #[clap(short, long, id = "OUTPUT_FILE")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read from input
    let gvas = match args.input {
        None => from_reader(std::io::stdin()),
        Some(input) => from_reader(File::open(input)?),
    }?;

    // Write to output
    match args.output {
        None => to_writer(std::io::stdout(), gvas),
        Some(output) => to_writer(File::create(output)?, gvas),
    }
}

fn from_reader<R: Read>(reader: R) -> Result<GvasFile> {
    let reader = BufReader::new(reader);
    Ok(serde_yaml::from_reader(reader)?)
}

fn to_writer<W: Write>(writer: W, gvas: GvasFile) -> Result<()> {
    let mut writer = BufWriter::new(writer);
    // WORKAROUND: GvasFile requires Seek attribute
    // Ok(gvas.write(&mut writer)?)
    let buf = Vec::new();
    let mut cursor = Cursor::new(buf);
    gvas.write(&mut cursor)?;
    Ok(writer.write_all(cursor.get_ref())?)
    // END WORKAROUND
}
