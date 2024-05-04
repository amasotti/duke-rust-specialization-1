use std::collections::HashMap;
use std::hash::Hash;

pub fn test_hashmap() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(String::from("Rust Programming"), String::from("Great Book"));
    reviews.insert(
        String::from("Kotlin Programming"),
        String::from("Great Book"),
    );
    reviews.insert(String::from("Java Programming"), String::from("Good Book"));
    reviews.insert(String::from("C# Programming"), String::from("Okay Book"));
    reviews.insert(String::from("pHP Programming"), String::from("Bad Book"));

    //println!("{:?}", reviews);

    for (key, value) in &reviews {
        //println!("{} -- was reviewed as -- {}", key, value);
        println!("{} -- was reviewed as -- {}", key, review_to_stars(value));
    }

    let book = "Some book";
    println!("{} => {}", book, get_review(book, reviews));
}

fn review_to_stars(review: &str) -> &str {
    match review {
        "Great Book" => "*****",
        "Good Book" => "****",
        "Okay Book" => "***",
        "Bad Book" => "**",
        _ => "Not Rated",
    }
}

fn get_review(book: &str, map: HashMap<String, String>) -> String {
    match map.get(book) {
        Some(review) => review.clone(),
        None => "Not Rated".to_string(),
    }
}
