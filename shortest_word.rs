// Define a function that takes a string as a parameter and returns an Option<&str>
fn shortest_word(s: &str) -> Option<&str> {
    // Split the string by whitespace and create an iterator over the words
    let words = s.split_whitespace();
    // Find the word with the smallest length using the min_by_key method
    let shortest = words.min_by_key(|w| w.len());
    // Return the shortest word as an Option<&str>
    shortest
}

// Test the function with some examples
fn main() {
    let s1 = "This is a string of words";
    let s2 = "Hello world";
    let s3 = "ea e rao";
    println!("The shortest word in {:?} is {:?}", s1, shortest_word(s1));
    println!("The shortest word in {:?} is {:?}", s2, shortest_word(s2));
    println!("The shortest word in {:?} is {:?}", s3, shortest_word(s3));
}
