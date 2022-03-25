use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();
    for word in magazine {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    for word in note {
        let count = words.entry(word).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false
        }
    }
    true
}
