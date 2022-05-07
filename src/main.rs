use std::{path::{PathBuf, Path}, fs::{File, write}};

use gumdrop::Options;
use xmlem::Document;

#[derive(Debug, Options)]
struct Args {
    #[options(help = "display help information")]
    help: bool,

    #[options(help = "output to file")]
    output_path: Option<PathBuf>,

    #[options(help = "replace input file with output")]
    is_replace: bool,

    #[options(free, help = "path to XML document")]
    xml_document_path: Option<PathBuf>,
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

    let text = prettify(&input_path)?;

    if let Some(path) = output_path {
        write(path, text)?;
    } else {
        println!("{}", text);
    }

    Ok(())
}

fn prettify(path: &Path) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let doc = Document::from_file(file)?;
    Ok(doc.to_string_pretty())
}