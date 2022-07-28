use std::{
    fs::{write, File},
    path::{Path, PathBuf},
};

use anyhow::Context;
use gumdrop::Options;
use xmlem::{display, Document};

#[derive(Debug, Options)]
struct Args {
    #[options(help = "display help information")]
    help: bool,

    #[options(free, help = "path to XML document")]
    xml_document_path: Option<PathBuf>,

    #[options(help = "output to file")]
    output_path: Option<PathBuf>,

    #[options(short = "r", long = "replace", help = "replace input file with output")]
    is_replace: bool,

    #[options(help = "number of spaces to indent (default: 2)")]
    indent: Option<usize>,

    #[options(short = "l", help = "max line length (default: 120)")]
    max_line_length: Option<usize>,

    #[options(
        short = "H",
        long = "hex-entities",
        help = "Use hex entity encoding (e.g. &#xNNNN;) for all entities"
    )]
    uses_hex_entities: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args_default_or_exit();

    let input_path = if let Some(path) = args.xml_document_path {
        path
    } else {
        eprintln!("ERROR: No XML document provided.");
        eprintln!("Run with -h for usage information.");
        return Ok(());
    };

    let output_path = if args.is_replace {
        Some(input_path.clone())
    } else if let Some(path) = args.output_path {
        Some(path)
    } else {
        None
    };

    let text = prettify(
        &input_path,
        args.indent,
        args.max_line_length,
        args.uses_hex_entities,
    )
    .with_context(|| format!("Failed to prettify '{}'", input_path.display()))?;

    if let Some(path) = output_path {
        write(&path, text).with_context(|| format!("Failed to write to '{}'", path.display()))?;
    } else {
        println!("{}", text);
    }

    Ok(())
}

fn prettify(
    path: &Path,
    indent: Option<usize>,
    max_line_length: Option<usize>,
    uses_hex_entities: bool,
) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let doc = Document::from_file(file)?;
    Ok(doc.to_string_pretty_with_config(&display::Config {
        is_pretty: true,
        indent: indent.unwrap_or(2),
        max_line_length: max_line_length.unwrap_or(120),
        entity_mode: if uses_hex_entities {
            display::EntityMode::Hex
        } else {
            display::EntityMode::Standard
        },
        indent_text_nodes: false,
    }))
}
