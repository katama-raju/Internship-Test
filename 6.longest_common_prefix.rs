fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();

    for (i, c) in strings[0].chars().enumerate() {
        if strings[1..].iter().all(|s| s.chars().nth(i) == Some(c)) {
            prefix.push(c);
        } else {
            break;
        }
    }

    prefix
}

fn main() {
    let strings = vec!["flower", "flow", "flight"];
    let common_prefix = longest_common_prefix(&strings);
    println!("Longest Common Prefix: {}", common_prefix);

    let other_strings = vec!["dog", "racecar", "car"];
    let other_prefix = longest_common_prefix(&other_strings);
    println!("Longest Common Prefix: {}", other_prefix);
}
