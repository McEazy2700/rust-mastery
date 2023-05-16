use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yello"), 50);
    let score = match scores.get("Blue").copied() {
        Some(value) => value,
        None => 0
    };
    println!("Score => Blue: {score}");
    for (key, value) in &scores {
        println!("Team {key}: {value}")
    }
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Red")).or_insert(35);
    println!("{:?}", scores);


    let text = "Oh what a wonderful world, yeah what a wonderful world";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word map: {:?}", word_map)
}
