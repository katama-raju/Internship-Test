fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); 
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>(); 
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let test_string1 = "A man, a plan, a canal, Panama";
    let test_string2 = "hello world";

    if is_palindrome(test_string1) {
        println!("'{}' is a palindrome.", test_string1);
    } else {
        println!("'{}' is not a palindrome.", test_string1);
    }

    if is_palindrome(test_string2) {
        println!("'{}' is a palindrome.", test_string2);
    } else {
        println!("'{}' is not a palindrome.", test_string2);
    }
}
