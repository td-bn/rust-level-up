#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_map = HashMap::new();

    for word in magazine {
        word_map.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }

    for word in note {
        let v = word_map.entry(word).and_modify(|x| *x -= 1).or_insert(-1);
        if *v < 0 {
            return false;
        }
    }
    true
}
