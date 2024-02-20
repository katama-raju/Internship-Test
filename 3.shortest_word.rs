 fn shortest_word(s: &str) -> Option<&str> {
     let words = s.split_whitespace();
     let shortest = words.min_by_key(|w| w.len());
     shortest
}

 fn main() {
    let s1 = "This is a string of words";
    let s2 = "Hello world";
    let s3 = "ea e rao";
    println!("The shortest word in {:?} is {:?}", s1, shortest_word(s1));
    println!("The shortest word in {:?} is {:?}", s2, shortest_word(s2));
    println!("The shortest word in {:?} is {:?}", s3, shortest_word(s3));
}
