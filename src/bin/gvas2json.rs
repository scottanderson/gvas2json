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

    /// Pretty-print JSON output.
    #[clap(short, long, id = "PRETTY", default_value_t = true)]
    pretty: bool,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Read from input
    let gvas = match args.input {
        None => from_reader(std::io::stdin()),
        Some(input) => from_reader(File::open(input)?),
    }?;

    // Transcode the data
    let json = match args.pretty {
        true => serde_json::to_string_pretty(&gvas),
        false => serde_json::to_string(&gvas),
    }?;

    // Write to output
    match args.output {
        None => to_writer(std::io::stdout(), json.as_bytes()),
        Some(output) => to_writer(File::create(output)?, json.as_bytes()),
    }
}

fn from_reader<R: Read>(input: R) -> Result<GvasFile> {
    let mut input = BufReader::new(input);
    // WORKAROUND: GvasFile requires Seek attribute
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    let mut input = Cursor::new(buf);
    // END WORKAROUND
    Ok(GvasFile::read(&mut input)?)
}

fn to_writer<W: Write>(writer: W, output: &[u8]) -> Result<()> {
    let mut writer = BufWriter::new(writer);
    Ok(writer.write_all(output)?)
}
