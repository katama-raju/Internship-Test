 fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() {
    let original_string = "Hi guys";
    let reversed_string = reverse_string(original_string);
    
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}
