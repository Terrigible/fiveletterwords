use std::{collections::{HashMap, HashSet}, fs::{self, File, OpenOptions}, io::Write};

fn word_to_bitmask(word: &str) -> u32{
    let mut alphabet_mask: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
    let mut bit_selector: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    let word_chars = word.chars().collect::<HashSet<char>>();
    for alphabet in "abcdefghijklmnopqrstuvwxyz".chars(){
        if word_chars.contains(&alphabet) {
            alphabet_mask |= bit_selector;
        }
        bit_selector = bit_selector >> 1;
    }
    alphabet_mask
}

fn main() {
    let words_string = fs::read_to_string("words_alpha.txt").expect("Could not read file").replace("\r\n", "\n");
    let words = words_string.split("\n");
    let five_letter_words = words.clone().filter(|word| word.len() == 5);
    let five_unique_letter_words = five_letter_words.filter(|word| HashSet::<char>::from_iter(word.chars()).len() == 5);
    let mut unique_letter_set_words = Vec::<&str>::new();
    let mut bitmask_set = HashSet::<u32>::new();
    let bitmask_vec: Vec<(&str, u32)> = five_unique_letter_words.clone().map(|word| (word, word_to_bitmask(word))).collect();
    for (word, mask) in &bitmask_vec {
        if bitmask_set.contains(mask){
            continue;
        }
        unique_letter_set_words.push(word);
        bitmask_set.insert(*mask);
    }
    let bitmask_vec: Vec<(&str, u32)> = bitmask_vec.iter().copied().filter(|(word, _)| unique_letter_set_words.contains(word)).collect();
    let bitmask_map = HashMap::<&str, u32>::from_iter(bitmask_vec.clone());
    let mut disjoint_hashmap:  HashMap<&str, Vec<&str>> = HashMap::new();
    for (word_1, mask_1) in &bitmask_vec {
        let mut disjoint_words: Vec<&str> = Vec::new();
        for (word_2, mask_2) in &bitmask_vec {
            if mask_1 & mask_2 == 0 {
                disjoint_words.push(word_2)
            }
        }
        disjoint_hashmap.insert(word_1, disjoint_words);
    }
    let mut found_combos = Vec::new();
    File::create("output.txt").unwrap();
    let mut file
    = OpenOptions::new()
    .append(true)
    .open("output.txt")
    .unwrap();
    let n_words = unique_letter_set_words.iter().clone().count();
    for (i, &word_1) in unique_letter_set_words.iter().enumerate(){
        println!("{} {}/{}", word_1, i+1, n_words);
        let mask_1 = bitmask_map.get(word_1).unwrap();
        let word_2_set
        = bitmask_vec.iter().filter_map(|(word, mask)| if mask&mask_1 == 0 {Some(*word)} else {None});
        for word_2 in word_2_set {
            let mask_2 = mask_1|bitmask_map.get(word_2).unwrap();
            let word_3_set
            = bitmask_vec.iter().filter_map(|(word, mask)| if mask&mask_2 == 0 {Some(*word)} else {None});
            for word_3 in word_3_set {
                let mask_3 = mask_2|bitmask_map.get(word_3).unwrap();
                let word_4_set
                = bitmask_vec.iter().filter_map(|(word, mask)| if mask&mask_3 == 0 {Some(*word)} else {None});
                for word_4 in word_4_set {
                    let mask_4 = mask_3|bitmask_map.get(word_4).unwrap();
                    let word_5_set
                    = bitmask_vec.iter().filter_map(|(word, mask)| if mask&mask_4 == 0 {Some(*word)} else {None});
                    for word_5 in word_5_set {
                        let word_set = HashSet::<&str>::from_iter([word_1, word_2, word_3, word_4, word_5]);
                        if found_combos.contains(&word_set){
                            continue;
                        }
                        println!("{}, {}, {}, {}, {}", word_1, word_2, word_3, word_4, word_5);
                        found_combos.push(HashSet::from_iter([word_1, word_2, word_3, word_4, word_5]));
                        writeln!(file, "{}, {}, {}, {}, {}", word_1, word_2, word_3, word_4, word_5).unwrap();
                    }
                }
            }
        }
    }
}