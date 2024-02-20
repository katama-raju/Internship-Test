fn is_palindrome(input: &str) -> bool {
    let cleaned_input: String = input.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed_input: String = cleaned_input.chars().rev().collect();

    cleaned_input.eq_ignore_ascii_case(&reversed_input)
}

fn main() {
    let example_palindrome = "A man, a plan, a canal, Panama!";
    let example_not_palindrome = "Hello, World!";

    if is_palindrome(example_palindrome) {
        println!("The string is a palindrome");
    } else {
        println!("The string is not a palindrome");
    }

    if is_palindrome(example_not_palindrome) {
        println!("The string is a palindrome");
    } else {
        println!("The string is not a palindrome");
    }
}
