use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut charmap = HashMap::new();
        chars.chars().for_each(|c| {
            charmap.entry(c).and_modify(|x| *x += 1).or_insert(1);
        });
        words
            .iter()
            .map(|word| {
                let mut map = HashMap::new();
                word.chars().for_each(|c| {
                    map.entry(c).and_modify(|x| *x += 1).or_insert(1);
                });
                (word.len(), map)
            })
            .filter_map(|(n, map)| {
                if map.iter().all(|(k, v)| {
                    if let Some(x) = charmap.get(&k) {
                        *v <= *x
                    } else {
                        false
                    }
                }) {
                    Some(n as i32)
                } else {
                    None
                }
            })
            .sum::<i32>()
    }
}

