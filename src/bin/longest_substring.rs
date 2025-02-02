use std::collections::HashMap;

fn main() {
    let result = length_of_longest_substring("pwwkew".to_string());
    println!("Longest substring length: {}", result);
}
/// enumerates and loops over the string, check if the character is known by quering a hashmap 
/// if not known current_length and max length increases by `char index + 1` (no match found so just count the number of chars encounterred so far)
/// and the character is inserted into the hasmap with chars as key and index as value. 
/// if character is found in hashmap we reset our slider to the length of the found chars in the string(setting our position back to zero) while max length
/// remains at previous length 

fn length_of_longest_substring(s: String) -> usize {
    let mut haspmap = HashMap::new();
    // stores the longest string 
    let mut max_length = 0;
    // keeps the position of the next match
    let mut found_slider = 0;
    // match each character to the index of its first occurence as define by `found_slider`
    let mut char_index = HashMap::new();

    for (i, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_index.get(&c) {
            found_slider = prev_index + 1;
        }
        let current_length = i - found_slider + 1;
        max_length = max_length.max(current_length);
        char_index.insert(c, i);
        haspmap.insert(i, &s[found_slider..=i]);
    }

    println!("{:?}", haspmap.values());

    max_length
}

