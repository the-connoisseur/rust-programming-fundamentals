use std::io;

fn main() {
    let palindrome_checkers: [fn(s: &str) -> bool; 2] = [is_pal_loop, is_pal_recursion];
    println!("This is a palindrome checker program.");
    println!("A palindrome is a word, phrase, or number that reads the same forward and backward.");
    println!("Strings \"Kayak\", \"race car\", \"161\" are all palindromes.");
    // The main loop keeps the program running until the user chooses to exit.
    'main: loop {
        println!("Enter a string, or E (to exit)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input.");
        if input.trim() == "E" {
            println!("Exiting the palindrome checker program.");
            break 'main;
        }

        // Remove the trailing newline character that comes with the input.
        if input.ends_with('\n') {
            input.pop();
        }

        // There is no point in continuing if there are no alphanumeric characters.
        let sanitized_input = remove_non_alphanumeric(input.as_str());
        if sanitized_input.is_empty() {
            println!(
                "[WARNING]: string \"{}\" does not have any alphanumeric characters, ignored.",
                input
            );
            continue 'main;
        }

        // Check for palindromes with each of our implementations.
        for func in palindrome_checkers.iter() {
            // First check if the whole sentence is a palindrome.
            if func(&sanitized_input) {
                println!("String \"{}\" is a palindrome.", input);
            } else {
                // If it isn't, check each word.
                let words: Vec<&str> = input.split_whitespace().collect();
                let mut pal_words: Vec<String> = vec![];
                for word in words {
                    let sanitized_word = remove_non_alphanumeric(word);
                    if sanitized_word.is_empty() {
                        continue;
                    }
                    if func(&sanitized_word) {
                        pal_words.push(sanitized_word);
                    }
                }
                if pal_words.is_empty() {
                    println!("String \"{}\" is NOT a palindrome.", input);
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
                }
            }
        }
        println!("===");
    }
}

// Returns true if the &char is an ASCII alphanumeric character.
fn is_alphanumeric(c: &char) -> bool {
    c.is_ascii_alphanumeric()
}

// Returns a sanitized input string with non-alphanumeric characters removed.
fn remove_non_alphanumeric(s: &str) -> String {
    s.chars().filter(is_alphanumeric).collect()
}

// Checks if the input string is a palindrome by iterating over its characters from both directions
// simultaneously and comparing them.
fn is_pal_loop(s: &str) -> bool {
    let lower_s = s.to_lowercase();
    let chars = lower_s.chars();
    let mut reverse_chars = chars.clone().rev();
    for c in chars {
        if c != reverse_chars.next().unwrap() {
            return false;
        }
    }
    true
}

// Checks if the input string is a palindrome by recursively comparing the first and last
// characters and then subsampling the string.
fn is_pal_recursion(s: &str) -> bool {
    let lower_s = s.to_lowercase();
    let length = lower_s.chars().count();
    if length < 2 {
        return true;
    }
    if lower_s.chars().next() == lower_s.chars().last() {
        return is_pal_recursion(&lower_s[1..length - 1]);
    }
    false
}
