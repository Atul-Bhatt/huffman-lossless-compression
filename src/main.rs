use std::fs;

fn main() {
    let input = fs::read_to_string("src/input/les-miserables.txt").expect("Cannot read file.");
    let mut character_frequency = [0; 256];
    

    // calculate the frequency of each character
    for ch in input.chars() {
        if (ch as usize) > 256 {
            match ch as usize {
                8220 | 8221 => character_frequency[34] += 1,
                8217 => character_frequency[39] += 1,
                8212 => character_frequency[45] += 1, 
                _ => {}
            }
        } else {
            character_frequency[ch as usize] += 1;
        }
    }

    // check whether x appears 333 times and t occurs 223000 times
    assert_eq!(333, character_frequency['X' as usize]);
    assert_eq!(223000, character_frequency['t' as usize]);
}
