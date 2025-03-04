use clap::{Arg, ArgAction, Command};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

// Holds command-line options.
#[derive(Debug)]
struct Options {
    chars: bool,
    words: bool,
    lines: bool,
}

impl Options {
    // Returns true if none of the options are specified.
    fn is_none(&self) -> bool {
        !self.chars && !self.words && !self.lines
    }
}

// Represents a file or directory.
enum Entry {
    File {
        name: String,
        char_count: Option<usize>,
        word_count: Option<usize>,
        line_count: Option<usize>,
        error: Option<String>,
    },
    Directory {
        name: String,
        contents: Vec<Entry>,
    },
}

// Builds an entry for the given path and options. is_top_level controls how to treat empty file
// entries; only top-level files get an error.
fn build_entry(path: &Path, options: &Options, is_top_level: bool) -> Entry {
    if path.is_file() {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                return Entry::File {
                    name: path.to_string_lossy().to_string(),
                    char_count: None,
                    word_count: None,
                    line_count: None,
                    error: Some(e.to_string()),
                };
            }
        };

        if is_top_level && content.is_empty() && !options.is_none() {
            return Entry::File {
                name: path.to_string_lossy().to_string(),
                char_count: None,
                word_count: None,
                line_count: None,
                error: Some("File is empty".to_string()),
            };
        }

        let char_count = if options.chars {
            Some(content.chars().count())
        } else {
            None
        };
        let word_count = if options.words {
            Some(content.split_whitespace().count())
        } else {
            None
        };
        let line_count = if options.lines {
            Some(content.lines().count())
        } else {
            None
        };
        Entry::File {
            name: path.to_string_lossy().to_string(),
            char_count,
            word_count,
            line_count,
            error: None,
        }
    } else if path.is_dir() {
        // With directories, the strategy is to collect the immediate contents of the directory by
        // recursively building an entry for each item.
        let mut contents = Vec::new();
        for result in WalkDir::new(path).max_depth(1).into_iter() {
            if let Ok(entry) = result {
                if entry.path() == path {
                    // Skip the directory itself.
                    continue;
                }
                contents.push(build_entry(entry.path(), options, false));
            }
        }
        Entry::Directory {
            name: path.to_string_lossy().to_string(),
            contents,
        }
    } else {
        Entry::File {
            name: path.to_string_lossy().to_string(),
            char_count: None,
            word_count: None,
            line_count: None,
            error: Some("Invalid path".to_string()),
        }
    }
}

// Prints a file entry.
fn print_file(entry: &Entry, options: &Options) {
    if let Entry::File {
        name,
        char_count,
        word_count,
        line_count,
        error,
    } = entry
    {
        if let Some(err) = error {
            if err == "File is empty" {
                println!("File \"{}\" is empty", name);
            } else {
                println!("File name: \"{}\", error: {}", name, err);
            }
        } else {
            print!("File name: \"{}\"", name);
            if options.chars {
                print!(", char count: {}", char_count.unwrap_or(0));
            }
            if options.words {
                print!(", word count: {}", word_count.unwrap_or(0));
            }
            if options.lines {
                print!(", line count: {}", line_count.unwrap_or(0));
            }
            println!();
        }
    }
}

// Prints a directory entry recursively.
fn print_directory(entry: &Entry, options: &Options) {
    if let Entry::Directory { name, contents } = entry {
        if contents.is_empty() {
            println!("Directory \"{}\" is empty", name);
        } else {
            println!("Directory name: \"{}\"", name);
            for content in contents {
                match content {
                    Entry::File { .. } => print_file(content, options),
                    Entry::Directory { .. } => print_directory(content, options),
                }
            }
        }
    }
}

// Parses command-line arguments and returns all matches.
fn parse_args() -> clap::ArgMatches {
    Command::new("cwl")
        .version("1.0.0")
        .about("cwl command")
        .arg(
            Arg::new("path")
                .required(true)
                .help("path to a file or directory"),
        )
        .arg(
            Arg::new("chars")
                .short('c')
                .long("chars")
                .action(ArgAction::SetTrue)
                .help("get the character count"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .action(ArgAction::SetTrue)
                .help("get the word count"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .action(ArgAction::SetTrue)
                .help("get the line count"),
        )
        .get_matches()
}

fn main() {
    let args = parse_args();

    let path_str = args.get_one::<String>("path").unwrap();
    let path = Path::new(path_str);
    let options = Options {
        chars: args.get_flag("chars"),
        words: args.get_flag("words"),
        lines: args.get_flag("lines"),
    };

    if !path.exists() {
        eprintln!("[ERROR] Invalid path \"{}\"", path.display());
        return;
    }

    // If no options were provided, just print the path and return.
    if options.is_none() {
        if path.is_file() {
            println!("File name: \"{}\"", path.display());
        } else if path.is_dir() {
            println!("Directory name: \"{}\"", path.display());
        }
    } else {
        // Process the file or directory with options.
        if path.is_file() {
            let entry = build_entry(path, &options, true);
            print_file(&entry, &options);
        } else if path.is_dir() {
            let entry = build_entry(path, &options, false);
            print_directory(&entry, &options);
        }
    }
}
