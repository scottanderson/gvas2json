use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored_json::{ColorMode, Output};
use gvas::GvasFile;
use minus::Pager;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, IsTerminal, Read, Write};

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

    /// Pretty-print JSON output.
    #[clap(short, long, id = "PRETTY", default_value_t = true)]
    pretty: bool,

    /// Enable colored output.
    #[clap(long, id = "COLOR", default_value = "auto")]
    color: WhenValues,

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

#[derive(ValueEnum, Clone, Debug)]
enum WhenValues {
    /// Always show colored output.
    Always,
    /// Automatically detect if terminal supports colors.
    Auto,
    /// Never show colored output.
    Never,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Check for terminal color support
    let color_mode = match args.color {
        WhenValues::Always => ColorMode::On,
        WhenValues::Auto => match args.output {
            None => {
                // Auto needs to be resolved before creating the pager
                match ColorMode::Auto(Output::StdOut).use_color() {
                    true => ColorMode::On,
                    false => ColorMode::Off,
                }
            }
            _ => ColorMode::Off,
        },
        WhenValues::Never => ColorMode::Off,
    };

    // Parse type hint arguments
    let types = parse_types(args.types);

    // Read from input
    let gvas = match args.input {
        None => from_reader(std::io::stdin(), &types),
        Some(input) => from_reader(File::open(input)?, &types),
    }?;

    // Transcode the data
    let json = format_json(&gvas, color_mode, args.pretty)?;

    // Write to output
    match args.output {
        None => {
            if args.no_pager || !std::io::stdout().is_terminal() {
                // Write directly to stdout
                to_writer(std::io::stdout(), json.as_bytes())
            } else {
                // Set up the pager
                let pager = Pager::new();
                pager.push_str(json.as_str())?;
                minus::page_all(pager)?;
                Ok(())
            }
        }
        Some(output) => to_writer(File::create(output)?, json.as_bytes()),
    }
}

fn format_json(gvas: &GvasFile, color_mode: ColorMode, pretty: bool) -> Result<String> {
    let json = if color_mode.use_color() {
        let value = serde_json::to_value(gvas)?;
        colored_json::to_colored_json(&value, color_mode)?
    } else if pretty {
        serde_json::to_string_pretty(gvas)?
    } else {
        serde_json::to_string(gvas)?
    };
    Ok(json)
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
    const LINE_ENDING: &[u8] = "\n".as_bytes();
    if !output.ends_with(LINE_ENDING) {
        writer.write_all(LINE_ENDING)?;
    }
    Ok(())
}
