use anyhow::Result;
use clap::Parser;
use gvas::GvasFile;
use minus::Pager;
use std::collections::HashMap;
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

    /// Type hints as key-value pairs.
    #[clap(short = 't', long = "type")]
    types: Vec<String>,

    /// Disable pager.
    #[clap(long, id = "NO_PAGER")]
    no_pager: bool,
}

fn parse_types(args: Vec<String>) -> HashMap<String, String> {
    let mut result = HashMap::<String, String>::new();
    for arg in args {
        let parts: Vec<&str> = arg.split('=').collect();
        assert_eq!(parts.len(), 2, "Hint should be in the format key=value");
        let k = String::from(parts[0]);
        let v = String::from(parts[1]);
        result.insert(k, v);
    }
    result
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Parse type hint arguments
    let types = parse_types(args.types);

    // Read from input
    let gvas = match args.input {
        None => from_reader(std::io::stdin(), &types),
        Some(input) => from_reader(File::open(input)?, &types),
    }?;

    // Transcode the data
    let yaml = {
        let mut yaml = serde_yaml::to_string(&gvas)?;
        while yaml.ends_with("\n\n") {
            yaml.pop(); // Remove the last character from the string
        }
        yaml
    };

    // Write to output
    match args.output {
        None => {
            if args.no_pager {
                // Write directly to stdout
                to_writer(std::io::stdout(), yaml.as_bytes())
            } else {
                // Set up the pager
                let pager = Pager::new();
                pager.push_str(yaml.as_str())?;
                minus::page_all(pager)?;
                Ok(())
            }
        }
        Some(output) => to_writer(File::create(output)?, yaml.as_bytes()),
    }
}

fn from_reader<R: Read>(input: R, types: &HashMap<String, String>) -> Result<GvasFile> {
    let mut input = BufReader::new(input);
    // WORKAROUND: GvasFile requires Seek attribute
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    let mut input = Cursor::new(buf);
    // END WORKAROUND
    Ok(GvasFile::read_with_hints(&mut input, types)?)
}

fn to_writer<W: Write>(writer: W, output: &[u8]) -> Result<()> {
    let mut writer = BufWriter::new(writer);
    writer.write_all(output)?;
    Ok(())
}
