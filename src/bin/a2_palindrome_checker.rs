use std::io::{self, Write};

fn main() {
    let palindrome_checkers: [fn(s: &str) -> bool; 2] = [is_pal_loop, is_pal_recursion];
    println!("This is a palindrome checker program.");
    println!("A palindrome is a word, phrase, or number that reads the same forward and backward.");
    println!("Strings \"Kayak\", \"race car\", \"161\" are all palindromes.");
    // The main loop keeps the program running until the user chooses to exit.
    'main: loop {
        println!("Enter a string, or E (to exit)");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");
        if input.trim() == "E" {
            println!("Exiting the palindrome checker program.");
            io::stdout().flush().unwrap();
            break 'main;
        }

        // Remove the trailing newline character that comes with the input.
        if input.ends_with('\n') {
            input.pop();
        }

        // Remove the non-alphanumeric characters from the input string. Preserve the whitespace as
        // we may need it to discern the individual words.
        let alphanum_input: String = remove_non_alphanumeric(input.as_str());
        if alphanum_input.is_empty() {
            println!(
                "[WARNING]: string \"{}\" does not have any alphanumeric characters, ignored.",
                input
            );
            io::stdout().flush().unwrap();
            continue 'main;
        }

        // Convert all characters to lower case, as we want to treat it as case insensitive.
        let alphanum_input: String = alphanum_input.to_lowercase();

        // Check for palindromes with each of our implementations.
        for func in palindrome_checkers.iter() {
            // First check if the whole sentence is a palindrome.
            if func(alphanum_input.as_str()) {
                println!("String \"{}\" is a palindrome.", input);
                io::stdout().flush().unwrap();
            } else {
                // If it isn't, check each word.
                let words: Vec<&str> = alphanum_input.split_whitespace().collect();
                let mut pal_words: Vec<&str> = vec![];
                for word in words {
                    if func(word) {
                        pal_words.push(word);
                    }
                }
                if pal_words.is_empty() {
                    println!("String \"{}\" is NOT a palindrome.", input);
                    io::stdout().flush().unwrap();
                } else {
                    println!("String \"{}\" is NOT a palindrome, but has the following palindrome item(s) in it:", input);
                    let mut first: bool = true;
                    for pal_word in pal_words {
                        if first {
                            print!("  {}", pal_word);
                            first = false;
                        } else {
                            print!(", {}", pal_word);
                        }
                    }
                    println!(".");
                    io::stdout().flush().unwrap();
                }
            }
        }
        println!("===");
    }
}

// Returns true if the &char is an ASCII alphanumeric or ASCII whitespace character.
fn is_alphanumeric_or_whitespace(c: &char) -> bool {
    c.is_ascii_alphanumeric() || c.is_ascii_whitespace()
}

// Returns
fn remove_non_alphanumeric(s: &str) -> String {
    s.chars().filter(is_alphanumeric_or_whitespace).collect()
}

fn is_pal_loop(s: &str) -> bool {
    let chars = s.chars();
    let mut reverse_chars = chars.clone().rev();
    for c in chars {
        if c != reverse_chars.next().unwrap() {
            return false;
        }
    }
    true
}

fn is_pal_recursion(s: &str) -> bool {
    let length = s.chars().count();
    if length < 2 {
        return true;
    }
    if s.chars().next() == s.chars().last() {
        return is_pal_recursion(&s[1..length - 1]);
    }
    false
}
