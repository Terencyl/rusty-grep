use std::error::Error;
use std::io::IsTerminal;

use clap::Parser;
use colored::Colorize;
use regex::Regex;

#[derive(Parser)]
#[command(name = "rg")]
#[command(about = "A simple grep tool allowing to search for a pattern in files")]
#[command(author, version)]
struct Args {
    /// Pattern to search for
    #[arg(value_name = "PATTERN")]
    pattern: String,

    /// Files to search
    #[arg(value_name = "FILE")]
    file: Vec<String>,

    /// Show line numbers with output lines
    #[arg(short = 'n', long = "line-number")]
    show_line_numbers: bool,

    /// Ignore case in patterns and files data
    #[arg(short = 'i', long = "ignore-case")]
    case_insensitive: bool,

    /// Invert the matching so that it selects non-matching lines
    #[arg(short = 'v', long = "invert-match")]
    invert_match: bool,

    /// Output a count of matching lines for each file instead of the normal output
    #[arg(short = 'c', long = "count", conflicts_with_all = ["show_line_numbers", "files_with_matches"])]
    count: bool,

    /// Output the name of each file where matches were found instead of the normal output
    #[arg(short = 'l', long = "files-with-matches", conflicts_with_all = ["show_line_numbers", "count"])]
    files_with_matches: bool,

    /// Matches only lines containing the whole pattern, preceded or followed by non-word characters
    #[arg(short = 'w', long = "word-regexp")]
    whole_words: bool,

    #[arg(long = "color", default_value = "auto", value_name = "WHEN")]
    #[arg(value_parser = ["auto",  "always", "never"])]
    color: String,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("Application error: {e}");
        std::process::exit(1);
    }
}

/// Search for a pattern in files
fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let regex = get_regex(&args)?;
    let show_filename = args.file.len() > 1;

    for file in &args.file {
        if let Err(e) = process_file(file, &args, &regex, show_filename) {
            eprintln!("{}: {}", file, e);
        }
    }
    Ok(())
}

/// Create regex for highlighting and whole-word matching if needed
fn get_regex(args: &Args) -> Result<Regex, Box<dyn Error>> {
    let pattern = if args.whole_words {
        format!(r"\b{}\b", regex::escape(&args.pattern))
    } else {
        regex::escape(&args.pattern).to_string()
    };

    let regex = if args.case_insensitive {
        Regex::new(&format!(r"(?i){}", pattern))?
    } else {
        Regex::new(&pattern)?
    };

    Ok(regex)
}

/// Process a single file to find matching patterns inside of it and print them
fn process_file(
    file: &str,
    args: &Args,
    regex: &Regex,
    show_filename: bool,
) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(file)?;
    let mut count = 0;
    let use_color = should_use_color(&args.color);

    for (index, line) in contents.lines().enumerate() {
        if get_matches(args, line, regex) {
            if args.files_with_matches {
                println!("{file}");
                return Ok(());
            } else if args.count {
                count += 1;
            } else {
                print_match(
                    file,
                    index,
                    line,
                    regex,
                    args.show_line_numbers,
                    show_filename,
                    use_color,
                );
            }
        }
    }

    if args.count {
        if show_filename {
            println!("{file}:{count}");
        } else {
            println!("{count}");
        }
    }
    Ok(())
}

/// Check if a line matches the pattern
fn get_matches(args: &Args, line: &str, regex: &Regex) -> bool {
    let matches = regex.is_match(line);
    if args.invert_match { !matches } else { matches }
}

/// Print the matches of pattern in file to the output
fn print_match(
    file: &str,
    index: usize,
    line: &str,
    regex: &Regex,
    show_line_number: bool,
    show_filename: bool,
    use_color: bool,
) {
    let prefix = if show_filename {
        if show_line_number {
            format!("{file}:{}:", index + 1)
        } else {
            format!("{file}:")
        }
    } else {
        if show_line_number {
            format!("{}:", index + 1)
        } else {
            String::new()
        }
    };

    let output = if use_color {
        highlight_matches(line, regex)
    } else {
        line.to_string()
    };

    println!("{prefix}{output}");
}

fn should_use_color(color_option: &str) -> bool {
    match color_option {
        "always" => true,
        "never" => false,
        "auto" | _ => std::io::stdout().is_terminal(),
    }
}

fn highlight_matches(line: &str, regex: &Regex) -> String {
    let mut result = String::new();
    let mut last_match = 0;
    for mat in regex.find_iter(line) {
        // Append the line text before the match
        result.push_str(&line[last_match..mat.start()]);
        // Append the highlighted match
        result.push_str(&line[mat.start()..mat.end()].red().bold().to_string());
        // Update the last match position
        last_match = mat.end();
    }
    // Append the remaining text of the line after the last match
    result.push_str(&line[last_match..]);
    result
}
