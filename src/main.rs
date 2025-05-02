use rand::Rng;
use std::collections::HashSet;
use std::io;

fn main() {
    println!("Password Generator");

    println!("Please enter the desired password length (default: 24):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let password_length: usize = match input.trim() {
        "" => 24,
        input => match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number. Using default length (24).");
                24
            }
        },
    };

    println!("Do you want to exclude certain characters? (y/n, default: n)");
    let mut exclude_input = String::new();
    io::stdin()
        .read_line(&mut exclude_input)
        .expect("Failed to read input");

    let mut excluded_chars = HashSet::new();

    if exclude_input.trim().to_lowercase() == "y" {
        println!("Enter the characters to exclude (without spaces between characters):");
        let mut chars_input = String::new();
        io::stdin()
            .read_line(&mut chars_input)
            .expect("Failed to read input");

        for c in chars_input.trim().chars() {
            excluded_chars.insert(c);
        }
    }

    let password = generate_password(password_length, &excluded_chars);

    println!("Your generated password: {}", password);
    println!("Length of generated password: {}", password.len());
}

fn generate_password(length: usize, excluded_chars: &HashSet<char>) -> String {
    let mut rng = rand::thread_rng();
    let letters_lower = "abcdefghijklmnopqrstuvwxyz";
    let letters_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?/";

    let all_chars = format!(
        "{}{}{}{}",
        letters_lower, letters_upper, numbers, special_chars
    );

    let available_chars: Vec<char> = all_chars
        .chars()
        .filter(|c| !excluded_chars.contains(c))
        .collect();

    if available_chars.is_empty() {
        return String::from("Unable to generate password: all characters are excluded");
    }

    let mut password = String::with_capacity(length);

    if length >= 4 && !excluded_chars.contains(&'a') {
        password.push(
            letters_lower
                .chars()
                .nth(rng.gen_range(0..letters_lower.len()))
                .unwrap(),
        );
    }
    if length >= 4 && !excluded_chars.contains(&'A') {
        password.push(
            letters_upper
                .chars()
                .nth(rng.gen_range(0..letters_upper.len()))
                .unwrap(),
        );
    }
    if length >= 4 && !excluded_chars.contains(&'0') {
        password.push(
            numbers
                .chars()
                .nth(rng.gen_range(0..numbers.len()))
                .unwrap(),
        );
    }
    if length >= 4 && !excluded_chars.contains(&'!') {
        password.push(
            special_chars
                .chars()
                .nth(rng.gen_range(0..special_chars.len()))
                .unwrap(),
        );
    }

    while password.len() < length {
        let random_char = available_chars[rng.gen_range(0..available_chars.len())];
        password.push(random_char);
    }

    let mut password_chars: Vec<char> = password.chars().collect();
    // Fisher-Yates algorithm
    for i in (1..password_chars.len()).rev() {
        let j = rng.gen_range(0..=i);
        password_chars.swap(i, j);
    }

    password_chars.into_iter().collect()
}
