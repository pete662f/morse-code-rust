use std::io;

fn main() {
    println!("Enter your text you want to connvert to morse:");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to get string");

    for char in text.to_lowercase().chars() {
        match char {
            'a' => print!(".-"),
            'b' => print!("-.."),
            'c' => print!("-.-."),
            'd' => print!("-.."),
            'e' => print!("."),
            'j' => print!(".---"),
            'k' => print!("-.-"),
            'l' => print!(".-.."),
            'm' => print!("--"),
            'n' => print!("-."),
            'o' => print!("---"),
            'p' => print!(".--."),
            'q' => print!("--.-"),
            'r' => print!(".-."),
            's' => print!("..."),
            't' => print!("-"),
            'u' => print!("..-"),
            'v' => print!("...-"),
            'w' => print!(".--"),
            'x' => print!("-..-"),
            'y' => print!("-.--"),
            'z' => print!("--.."),
            'æ' => print!(".-.-"),
            'ø' => print!("---."),
            'å' => print!(".--.-"),
            '1' => print!(".----"),
            '2' => print!("..---"),
            '3' => print!("...--"),
            '4' => print!("....-"),
            '5' => print!("....."),
            '6' => print!("-...."),
            '7' => print!("--...."),
            '8' => print!("---.."),
            '9' => print!("----."),
            '0' => print!("-----"),
            '.' => print!(".-.-.-"),
            ',' => print!("--..--"),
            '?' => print!("..--.."),
            '\'' => print!(".----."),
            '!' => print!("-.-.--"),
            '/' => print!("-..-."),
            '(' => print!("-.--."),
            ')' => print!("-.--.-"),
            '&' => print!(".-..."),
            ':' => print!("---..."),
            ';' => print!("-.-.-."),
            '=' => print!("-...-"),
            '+' => print!(".-.-."),
            '-' => print!("-....-"),
            '_' => print!("..--.-"),
            '"' => print!(".-..-."),
            '$' => print!("...-..-"),
            '@' => print!(".--.-."),

            ' ' => print!(""),
            _ => continue,
        }
        print!("/");
    }
    println!();
}
