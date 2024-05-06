use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Replace white spaces with underscores
    //let sentence = sentence.replace(" ", "_");

    // Use slicing to get the first three characters of the sentence
    println!("{}", &sentence[0..4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // iterate over the characters in the sentence
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel: {}!", c),
            _ => continue,
        }
    }

    // Split and collect into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    let words_1 = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    // Just for me as quick check
    assert_eq!(words, words_1);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);

    let vowel_count = count_vowels(&sentence);
    println!("{:?}", vowel_count);
}


/// Count the number of vowels in a sentence
fn count_vowels(sentence: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
            _ => continue,
        }
    }
    map
}

