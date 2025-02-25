mod models;

use anyhow::{Context, Result};
use argh::FromArgs;
use models::Docs;
use rust_fuzzy_search::fuzzy_compare;
use std::io::{BufRead, Write};

#[derive(FromArgs, PartialEq, Debug)]
/// Nix function exploring
struct Cli {
    /// print out version and quit
    #[argh(switch)]
    version: bool,

    #[argh(subcommand)]
    command: Option<Command>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    List(ListCommand),
    Search(SearchCommand),
    Show(ShowCommand),
    Doc(DocCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Print out list of all functions
#[argh(subcommand, name = "list")]
struct ListCommand {
    /// print entries as serialized JSON to stdout
    #[argh(switch)]
    json: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Fuzzy search function titles
#[argh(subcommand, name = "search")]
struct SearchCommand {
    /// minimum match score for fuzzy matching
    #[argh(option, short = 't')]
    threshold: Option<f32>,

    /// query string to match against function titles
    #[argh(positional)]
    query: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Show function metadata
#[argh(subcommand, name = "show")]
struct ShowCommand {
    /// print entry as serialized JSON to stdout
    #[argh(switch)]
    json: bool,

    /// function path
    #[argh(positional)]
    path: Option<String>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Print markdown documentation for given function
#[argh(subcommand, name = "doc")]
struct DocCommand {
    /// function path
    #[argh(positional)]
    path: Option<String>,
}

const RAW_JSON: &str = include_str!("../data.json");

fn main() -> Result<()> {
    let cli: Cli = argh::from_env();

    if cli.version {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let Some(subcommand) = cli.command else {
        anyhow::bail!("No subcommand given");
    };

    let docs: Docs = serde_json::from_str(RAW_JSON).unwrap();
    let result = match subcommand {
        Command::List(args) => handle_list_command(args, docs)?,
        Command::Search(args) => handle_search_command(args, docs),
        Command::Show(args) => handle_show_command(args, docs)?,
        Command::Doc(args) => handle_doc_command(args, docs)?,
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

fn handle_list_command(args: ListCommand, docs: Docs) -> Result<String> {
    let stdout = if args.json {
        serde_json::to_string_pretty(&docs)?
    } else {
        docs.0
            .into_iter()
            .map(|func| func.meta.title)
            .collect::<Vec<_>>()
            .join("\n")
    };

    Ok(stdout)
}

fn handle_search_command(args: SearchCommand, docs: Docs) -> String {
    let mut matches = docs
        .iter()
        .filter_map(|entry| {
            let title = entry.meta.title.as_ref();
            let score = fuzzy_compare(&args.query, title);
            if score >= args.threshold.unwrap_or(0.3) {
                Some((title, score))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    matches.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    let mut stdout = String::new();
    for (content, _) in matches {
        stdout += content;
        stdout += "\n";
    }

    stdout
}

fn handle_show_command(args: ShowCommand, docs: Docs) -> Result<String> {
    let title = string_or_stdin(args.path)?;
    let entry = docs.get_by_title(title).context("no such function")?;

    if args.json {
        Ok(serde_json::to_string_pretty(entry)?)
    } else {
        Ok(format!("{:?}", entry))
    }
}

fn handle_doc_command(args: DocCommand, docs: Docs) -> Result<String> {
    let title = string_or_stdin(args.path)?;
    docs.get_by_title(title)
        .context("no such function")?
        .content
        .clone()
        .and_then(|source| source.content)
        .context("function has no documentation")
}
