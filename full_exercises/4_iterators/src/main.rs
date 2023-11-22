//This is an exercise on using iterators (https://doc.rust-lang.org/std/iter/trait.Iterator.html).
//The file "bee.txt" is read and various operations must be performed on the contents.
//All characters in the provided file are ascii characters. 
//The operations that must be implemented are:
//  - All non-alphabetical characters are discarded.
//  - All words need to be converted to lower case.
//  - The corruptions in the file must be removed. (corruptions are strings: "CORRUPTION")
//  - All Strings that correspond with the keys in `replace_map` must be replaced with their values.
//  - The keys in `words_to_count` must be counted and stored in the data structure.
//All the operations must be done in one pass using a single iterator, for loops are not allowed.
//The result must be a new String of all yielded words with spaces seperating them.
//hint: you'll have to convert the character iterator to a word iterator along the way.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let replace_map = HashMap::from([("barry", "larry"), ("stairs", "ramp"), ("yellow", "purple"), ("breakfast", "elevenses")]);
    let mut words_to_count = HashMap::from([("bee", 0), ("honey", 0), ("adam", 0), ("defenestration", 0)]);

    let mut file  = File::open("bee.txt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let new_contents = contents.iter()... //TODO




    println!("{}", new_contents);

    assert!(!new_contents.chars().any(|c| (!c.is_alphabetic() && c != ' ') || c.is_uppercase()));

    assert!(!new_contents.contains("corruption"));

    for (k, _) in replace_map {
        assert!(!new_contents.contains(&format!(" {} ", k)), "found: {}", k);
    }

    assert_eq!(words_to_count["bee"], 142);
    assert_eq!(words_to_count["honey"], 79);
    assert_eq!(words_to_count["adam"], 158);
    assert_eq!(words_to_count["defenestration"], 0);
    
    println!("Success");
}