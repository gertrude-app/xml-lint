use anyhow::Context;
use clap::{Parser, ValueHint};
use std::{path::PathBuf, str::FromStr};
use xmlem::{display, Document};

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about = "XML document linter")]
struct Args {
  /// Paths to XML documents
  #[clap(value_parser, value_hint = ValueHint::FilePath, required = true)]
  paths: Vec<PathBuf>,

  /// Replace input file with output
  #[clap(short = 'f', long)]
  fix: bool,

  /// Number of spaces to indent (default: 2)
  #[clap(long, value_parser)]
  indent: Option<usize>,

  /// Number of spaces to pad the end of an element without separate end-tag (default: 1)
  #[clap(short = 'e', value_parser)]
  end_pad: Option<usize>,

  /// Max line length (default: 120)
  #[clap(short = 'l', value_parser)]
  max_line_length: Option<usize>,

  /// Use hex entity encoding (e.g. &#xNNNN;) for all entities
  #[clap(short = 'x', long = "hex-entities")]
  uses_hex_entities: bool,

  /// Do not prettify and indent text nodes
  #[clap(short = 't', long = "text-indent")]
  text_indent: bool,
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let mut failures = Vec::new();
  for path in &args.paths {
    let xml = std::fs::read_to_string(path)
      .with_context(|| format!("Failed to read file: `{}`", path.display()))?;
    let doc = Document::from_str(&xml)
      .with_context(|| format!("Failed to parse xml from file: `{}`", path.display()))?;
    let prettified = prettify(&doc, &args);
    if args.fix {
      std::fs::write(path, prettified)
        .with_context(|| format!("Failed to write file: `{}`", path.display()))?;
    } else if prettified != xml {
      failures.push(path.to_string_lossy().to_string());
    }
  }
  if failures.is_empty() {
    eprintln!("All files are properly formatted.");
    Ok(())
  } else {
    eprintln!("xml-lint failed with invalid files:");
    for failure_path in failures {
      eprintln!("  - {failure_path}");
    }
    std::process::exit(1)
  }
}

fn prettify(doc: &Document, args: &Args) -> String {
  doc.to_string_pretty_with_config(
    &display::Config::default_pretty()
      .indent(args.indent.unwrap_or(2))
      .end_pad(args.end_pad.unwrap_or(1))
      .max_line_length(args.max_line_length.unwrap_or(120))
      .entity_mode(if args.uses_hex_entities {
        display::EntityMode::Hex
      } else {
        display::EntityMode::Standard
      })
      .indent_text_nodes(args.text_indent),
  )
}
