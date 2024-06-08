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
    let five_letter_words = words.filter(|word| word.len() == 5);
    let five_unique_letter_words = five_letter_words.filter(|word| HashSet::<char>::from_iter(word.chars()).len() == 5);
    let mut unique_letter_set_words = Vec::<&str>::new();
    let mut bitmask_set = HashSet::<u32>::new();
    let bitmask_vec: Vec<(&str, u32)> = five_unique_letter_words.map(|word| (word, word_to_bitmask(word))).collect();
    for (word, mask) in &bitmask_vec {
        if bitmask_set.contains(mask){
            continue;
        }
        unique_letter_set_words.push(word);
        bitmask_set.insert(*mask);
    }
    let bitmask_vec: Vec<(&str, u32)> = unique_letter_set_words.iter().map(|&word| (word, word_to_bitmask(word))).collect();
    let mut bitmask_only_vec = bitmask_vec.clone().iter().copied().map(|(_, mask)| mask).collect::<Vec<u32>>();
    let reverse_bitmask_map = HashMap::<u32, &str>::from_iter(bitmask_vec.clone().iter().map(|&(word, mask)| (mask, word)));
    let mut found_combos = Vec::new();
    File::create("output.txt").unwrap();
    let mut file
    = OpenOptions::new()
    .append(true)
    .open("output.txt")
    .unwrap();
    let n_words = unique_letter_set_words.iter().count();
    for (i, &word_1) in unique_letter_set_words.iter().enumerate(){
        println!("{} {}/{}", word_1, i+1, n_words);
        let mask_1 = bitmask_only_vec.remove(0);
        let mask_2_set
        = bitmask_only_vec.iter().filter(|&mask| mask&mask_1 == 0);
        for mask_2 in mask_2_set {
            let cumul_mask_2 = mask_1|mask_2;
            let mask_3_set
            = bitmask_only_vec.iter().filter(|&mask| mask&cumul_mask_2 == 0);
            for mask_3 in mask_3_set {
                let cumul_mask_3 = cumul_mask_2|mask_3;
                let mask_4_set
                = bitmask_only_vec.iter().filter(|&mask| mask&cumul_mask_3 == 0);
                for mask_4 in mask_4_set {
                    let cumul_mask_4 = cumul_mask_3|mask_4;
                    let mask_5_set
                    = bitmask_only_vec.iter().filter(|&mask| mask&cumul_mask_4 == 0);
                    for mask_5 in mask_5_set {
                        let mask_set = HashSet::<u32>::from_iter([mask_1, *mask_2, *mask_3, *mask_4, *mask_5]);
                        if found_combos.contains(&mask_set){
                            continue;
                        }
                        found_combos.push(mask_set);
                        let word_1 = *reverse_bitmask_map.get(&mask_1).unwrap();
                        let word_2 = *reverse_bitmask_map.get(&mask_2).unwrap();
                        let word_3 = *reverse_bitmask_map.get(&mask_3).unwrap();
                        let word_4 = *reverse_bitmask_map.get(&mask_4).unwrap();
                        let word_5 = *reverse_bitmask_map.get(&mask_5).unwrap();
                        println!("{}, {}, {}, {}, {}", word_1, word_2, word_3, word_4, word_5);
                        writeln!(file, "{}, {}, {}, {}, {}", word_1, word_2, word_3, word_4, word_5).unwrap();
                    }
                }
            }
        }
    }
}