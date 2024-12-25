use std::env;
use std::process::exit;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <op> <text>", args[0]);
        exit(1);
    }
    let op = &args[1];
    let text = &args[2];


    let res = match op.as_str() {
        "reverse" => text.chars().rev().collect::<String>(),
        "invert" => text
            .chars()
            .map(invert)
            .collect::<String>(),
        "uppercase" => text.to_uppercase(),
        "no-spaces" => text
            .chars()
            .map(delete_space)
            .collect::<String>(),
        "leet" => text
            .chars()
            .map(leet)
            .collect::<String>(),
        "acronym" => text.split_whitespace()
            .map(|word| word.chars().next().unwrap().to_uppercase().to_string())
            .collect::<String>(),
        _ => {
            eprintln!("Invalid operation: {}", op);
            exit(1);
        }
    };

    println!("result: {res}");
}

fn invert(c: char) -> String {
    if c.is_uppercase() {
        c.to_lowercase().to_string()
    } else {
        c.to_uppercase().to_string()
    }
}

fn leet(c: char) -> String {
    match c {
        'a' | 'A' => '4'.to_string(),
        'e' | 'E' => '3'.to_string(),
        'i' | 'I' => '1'.to_string(),
        'o' | 'O' => '0'.to_string(),
        's' | 'S' => '5'.to_string(),
        't' | 'T' => '7'.to_string(),
        _ => c.to_string(),
    }
}

fn delete_space(c: char) -> String {
    match c {
        ' ' => "".to_string(),
        _ => c.to_string(),
    }
}
