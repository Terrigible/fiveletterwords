use std::{
    collections::{HashMap, HashSet},
    fs::{self, File, OpenOptions},
    io::Write,
};

fn word_to_bitmask(word: &str) -> u32 {
    let mut alphabet_mask: u32 = 0b0000_0000_0000_0000_0000_0000_0000_0000;
    let mut bit_selector: u32 = 0b1000_0000_0000_0000_0000_0000_0000_0000;
    let word_chars = word.chars().collect::<HashSet<char>>();
    for alphabet in "abcdefghijklmnopqrstuvwxyz".chars() {
        if word_chars.contains(&alphabet) {
            alphabet_mask |= bit_selector;
        }
        bit_selector >>= 1;
    }
    alphabet_mask
}

fn get_next_mask_set(bitmask_only_vec: &[u32], cumul_mask: &u32) -> Vec<u32> {
    bitmask_only_vec
        .iter()
        .filter(|&mask| mask & cumul_mask == 0)
        .copied()
        .collect()
}

fn main() {
    let words_string = fs::read_to_string("words_alpha.txt").expect("Could not read file");
    let words = words_string.split_whitespace();
    let five_unique_letter_words = words
        .filter(|word| (word.len() == 5) & (HashSet::<char>::from_iter(word.chars()).len() == 5));
    let mut unique_letter_set_words = Vec::<&str>::new();
    let mut bitmask_set = HashSet::<u32>::new();
    let bitmask_vec: Vec<(&str, u32)> = five_unique_letter_words
        .map(|word| (word, word_to_bitmask(word)))
        .collect();
    for (word, mask) in &bitmask_vec {
        if bitmask_set.contains(mask) {
            continue;
        }
        unique_letter_set_words.push(word);
        bitmask_set.insert(*mask);
    }
    let chars: Vec<char> = unique_letter_set_words
        .iter()
        .flat_map(|&word| word.chars())
        .collect();
    let char_counts = chars.iter().fold(HashMap::new(), |mut acc, &c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let mut char_counts_vec = char_counts.iter().collect::<Vec<_>>();
    char_counts_vec.sort_by_key(|&(_, &count)| count);
    println!("{:?}", char_counts_vec);
    unique_letter_set_words.sort_by_key(|word| word.chars().map(|c| char_counts[&c]).min());
    let bitmask_vec: Vec<(&str, u32)> = unique_letter_set_words
        .iter()
        .map(|&word| (word, word_to_bitmask(word)))
        .collect();
    let mut bitmask_only_vec = bitmask_vec
        .iter()
        .map(|(_, mask)| mask)
        .copied()
        .collect::<Vec<u32>>();
    let reverse_bitmask_map =
        HashMap::<u32, &str>::from_iter(bitmask_vec.iter().map(|&(word, mask)| (mask, word)));
    let mut file_buf = Vec::new();
    let first_letter_mask = word_to_bitmask(&char_counts_vec[0].0.to_string());
    let second_letter_mask = word_to_bitmask(&char_counts_vec[1].0.to_string());
    let first_two_letter_mask = first_letter_mask | second_letter_mask;
    for mask_1 in bitmask_only_vec
        .iter()
        .filter(|&&mask| mask & first_two_letter_mask != 0)
        .copied()
        .collect::<Vec<_>>()
    {
        bitmask_only_vec.retain(|&mask| mask != mask_1);
        let mut mask_2_set = get_next_mask_set(&bitmask_only_vec, &mask_1);
        for mask_2 in mask_2_set.clone() {
            mask_2_set.retain(|&mask| mask != mask_2);
            let mut mask_3_set = get_next_mask_set(&mask_2_set, &mask_2);
            for mask_3 in mask_3_set.clone() {
                mask_3_set.retain(|&mask| mask != mask_3);
                let mut mask_4_set = get_next_mask_set(&mask_3_set, &mask_3);
                for mask_4 in mask_4_set.clone() {
                    mask_4_set.retain(|&mask| mask != mask_4);
                    let mask_5_set = get_next_mask_set(&mask_4_set, &mask_4);
                    for mask_5 in mask_5_set {
                        let word_1 = *reverse_bitmask_map.get(&mask_1).unwrap();
                        let word_2 = *reverse_bitmask_map.get(&mask_2).unwrap();
                        let word_3 = *reverse_bitmask_map.get(&mask_3).unwrap();
                        let word_4 = *reverse_bitmask_map.get(&mask_4).unwrap();
                        let word_5 = *reverse_bitmask_map.get(&mask_5).unwrap();
                        println!("{}, {}, {}, {}, {}", word_1, word_2, word_3, word_4, word_5);
                        writeln!(
                            file_buf,
                            "{}, {}, {}, {}, {}",
                            word_1, word_2, word_3, word_4, word_5
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    File::create("output.txt").unwrap();
    let mut file = OpenOptions::new().append(true).open("output.txt").unwrap();
    file.write_all(&file_buf).unwrap();
}
