use std::collections::{HashMap, HashSet};
use std::io;

// The list of movie titles to use for this program.
const MOVIE_TITLES: &[&str] = &[
    "Love Actually",
    "STAR WARS",
    "From Russia With love",
    "Dr. Strangelove",
    "Bourne Ultimatum",
    "The fault in our stars",
    "Bourne supremacy",
    "A star is born",
    "Starsky and Hutch",
    "Star Trek",
    "Lover's Paradise",
    "A Christmas Star",
    "Chitty Chitty Bang Bang",
    "Ernest Saves Christmas",
    "A CHRISTMAS CAROL",
    "That Darn Cat !",
    "The Muppet Christmas Carol",
    "White Christmas",
    "Farenheit 451",
];

// '\x1b' is the escape character and signals the start of an ANSI sequence.
// '[' is a control sequence introducer (CSI) for terminal manipulation.
// 31 is the value for red, and 'm' is a function identifier for graphics (color).
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    // Parse the movie titles into a hash map of word counts.
    let word_counts = get_word_counts();

    'main: loop {
        // Read user input.
        println!("Enter your search word(s) separated with one or more spaces/tabs, or E (to exit), and hit the Enter key:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // Check if the user wants to exit the program.
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("e") {
            break 'main;
        }

        // If there are no words, prompt again.
        if trimmed.is_empty() {
            println!(
                "Your input \"{}\" had no words to search for.\u{1F914}",
                // Remove the carriage return and/or line feed characters from the end (Windows
                // usually has both - '\r\n').
                input.trim_end_matches(['\r', '\n'])
            );
            continue 'main;
        }

        // Put the user's words into a set to deduplicate.
        let unique_words: HashSet<String> = trimmed
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();

        // Simultaneously filter out words that aren't found in our hash map, and transform the
        // ones that are to have their first character capitalized.
        let mut results: Vec<(String, u32)> = unique_words
            .iter()
            .filter_map(|word| {
                word_counts
                    .get(word) // This returns an Option<&str>
                    .map(|count| (capitalize_first(word), *count)) // And this skips the None's
            })
            .collect();

        // Display the results to the user.
        if results.is_empty() {
            println!("Sorry, none of the words in your input \"{}\" were found in the movie titles.\u{1F61E}", input.trim_end_matches(['\r', '\n']));
        } else {
            results.sort_by(|a, b| a.0.cmp(&b.0));
            for (word, count) in results {
                println!(
                    "{:<10} {}{}{}",
                    word,
                    RED,
                    "*".repeat(count as usize),
                    RESET
                );
            }
            println!("\u{1F642}");
        }
    }
}

// Iterates the entries in the global constant MOVIE_TITLES and returns a hash map of words to
// their respective counts.
fn get_word_counts() -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    for title in MOVIE_TITLES {
        for word in title.split_whitespace() {
            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }
    word_counts
}

// Converts the first character of the input string to upper case and returns it.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
