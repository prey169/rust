/*
-----------------------------------------------------------------------
- Description
  - Given a list of words, group the words that are anagrams
- Tools
  - Hashmaps, Nexted Loops
-----------------------------------------------------------------------
*/

use std::collections::{btree_map::Keys, HashMap};

fn word_groupings(words_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash = HashMap::new();

    let mut chat_freq = vec![0; 26];

    for current_word in words_list {
        for c in current_word.to_lowercase().chars() {
            chat_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }
        let key = chat_freq
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>();
        word_hash
            .entry(key)
            .or_insert(Vec::new())
            .push(current_word);
        chat_freq = vec![0; 26];
    }

    for (key, value) in &word_hash {
        println!("key # {:?} value {:?}", key, value);
    }

    word_hash.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "sutpid".to_string(),
        "apple".to_string(),
        "apelp".to_string(),
    ];

    let grouping = word_groupings(words);

    let input_word = String::from("teh");
    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The group of the word is {:?}", i)
        }
    }
}
