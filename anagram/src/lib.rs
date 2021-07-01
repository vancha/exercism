use std::collections::HashMap;
use std::collections::HashSet;


///returns a hashmap with characters as a key, and how often they appear in word as value
fn word_to_hashmap(word: &str) -> HashMap<char, i32> {
    word.chars()
        .fold(std::collections::HashMap::new(), |mut hm, c| {
            //the return value will be a hashmap with char counts
            match hm.contains_key(&c) {
                //check if the char is already present
                true => {
                    //if so
                    let val = hm.entry(c).or_insert(0); //get the corresponding number
                    *val += 1; //increment it
                    hm //and return the hashmap for the next iteration
                }
                false => {
                    //if not
                    hm.insert(c, 1); //insert it and set it's occurence to 1
                    hm //return the hashmap for the next iteration
                }
            }
        })
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_anagram_hm = word_to_hashmap(&word.to_lowercase());
    possible_anagrams
        .iter()
        //.map(|word| word.to_lowercase())
        .filter(|possible_anagram| word_to_hashmap(possible_anagram.to_lowercase().as_str()) == word_anagram_hm && *possible_anagram != &word)//if words have the same number of characters, they must be anagrams?
        .map(|w| *w)
        .collect::<HashSet<&'a str>>()
}
