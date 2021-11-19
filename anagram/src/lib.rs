use std::collections::HashSet;
use std::collections::HashMap;

fn map_word(word: &str) -> HashMap<String, i32>  {
    let mut char_map = HashMap::new();
    word.chars().for_each(|x| { 
        let lx = x.to_lowercase().to_string();
        char_map.entry(lx).and_modify(|y| {*y += 1}).or_insert(1); 
    });
    return char_map;
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let word_map = map_word(word);

    possible_anagrams.iter().for_each( |x| {
      if word.to_lowercase().eq(&x.to_lowercase()) {
        return;
      }
      let t_map = map_word(x);
      if word_map.eq(&t_map) {
        set.insert(*x);
      }
    });
    return set;
}
