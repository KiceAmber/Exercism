use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sorted_string(&lowercase_word);

    possible_anagrams
        .iter()
        .filter(|&&elem| {
            let lowercase_elem = elem.to_lowercase();
            elem.len() == lowercase_elem.len()
                && lowercase_word != lowercase_elem
                && sorted_word == sorted_string(&lowercase_elem)
        })
        .cloned()
        .collect()
}

fn sorted_string(word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word.sort();
    word.into_iter().collect()
}
