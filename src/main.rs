mod models;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use models::{ArchivedDocs, Docs};
use std::io::{BufRead, Write};

impl From<&ArchivedDocs> for Docs {
    fn from(val: &ArchivedDocs) -> Self {
        rkyv::deserialize::<Docs, rkyv::rancor::Error>(val).unwrap()
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Nix function exploring
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print out list of all functions
    List {
        /// print entries as serialized JSON to stdout
        #[arg(short, long)]
        json: bool,
    },

    /// Fuzzy search function titles
    Search {
        /// query string to match against function titles
        query: String,
    },

    /// Show function metadata
    Show {
        /// print entry as serialized JSON to stdout
        #[arg(short, long)]
        json: bool,

        /// function path, if none is given it will read from stdin
        path: Option<String>,
    },

    /// Print markdown documentation for given function
    Doc {
        /// function path, if none is given it will read from stdin
        path: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let bytes = include_bytes!("../data");
    let docs = unsafe { rkyv::access_unchecked::<ArchivedDocs>(&bytes[..]) };

    let result = match cli.command {
        Command::List { json } => handle_list_command(json, docs)?,
        Command::Search { query } => handle_search_command(&query, docs),
        Command::Show { path, json } => handle_show_command(path, json, docs)?,
        Command::Doc { path } => handle_doc_command(path, docs)?,
    };

    Ok(std::io::stdout().write_all(result.as_bytes())?)
}

/// If `string` is `None` read single line from stdin
fn string_or_stdin(string: Option<String>) -> Result<String> {
    match string {
        Some(value) => Ok(value),
        None => {
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut buf = String::new();
            handle.read_line(&mut buf)?;
            Ok(buf.trim().to_string())
        }
    }
}

fn handle_list_command(json: bool, archive: &ArchivedDocs) -> Result<String> {
    let stdout = if json {
        let docs: Docs = archive.into();
        serde_json::to_string_pretty(&docs)?
    } else {
        archive
            .0
            .iter()
            .map(|doc| doc.meta.title.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(stdout)
}

fn handle_search_command(query: &str, archive: impl Into<Docs>) -> String {
    let docs = archive.into();
    let results = docs.fuzzy_search_sorted(query);
    let mut stdout = String::new();
    for (content, _) in results {
        stdout += &content.meta.title;
        stdout += "\n";
    }

    stdout
}

fn handle_show_command(
    path: Option<String>,
    json: bool,
    archive: impl Into<Docs>,
) -> Result<String> {
    let docs = archive.into();
    let title = string_or_stdin(path)?;
    let entry = docs.get_by_title(title).context("no such function")?;

    if json {
        Ok(serde_json::to_string_pretty(entry)?)
    } else {
        Ok(format!("{:?}", entry))
    }
}

fn handle_doc_command(path: Option<String>, archive: impl Into<Docs>) -> Result<String> {
    let docs = archive.into();
    let title = string_or_stdin(path)?;
    docs.get_by_title(title)
        .context("no such function")?
        .content
        .clone()
        .and_then(|source| source.content)
        .context("function has no documentation")
}
