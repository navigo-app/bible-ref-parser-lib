mod parsing;
mod title_hashmap;
mod bible_map;

use parsing::Parsing;

use std::vec;

fn main() {
    let test_refs: Vec<&str> = vec![
        "Gen 50:1-Exo 2:1;Psa 5:6,7",
        "Jn 3:16-17, Ge 1:1 - Ex 2:1, Lev 9:12",
        "2 Peter 1-1 John 2, Romans 5:12, 2 Corinthians 11:12-Galatians 1:5",
        "Acts 15-16",
        "Genesis 1:1, 2:3",
        "Psalm 119:150",
        "Rev. 22:1",
        "III John 1:3",
        "Gen. 1",
        "Exo. 1;2,3",
        "Dt 1:1,2,4",
        "Ge 1:1-2:20",
        "Gen 1:1-2:20,23",
        "Gen 1; Psalm 1",
        "I Cor. 1 & Ps. 1",
        "Judges 1 and Psalm 1",
        "Gen 7:21,19:17,20-27",
        "Mark 1,2,3:27",
        "Gen 1:1-20; 24",
        "2 Jn 1; 24:2; 39",
        "Psalm 6:22, 1 John 3:15, Gen 1:1",
        "GENESIS 7:21,19:17,20-27;1 John 1,2,3:27,Psalm 1,2:7",
        "Acts 2:3,16-17,33, 10:44-45, 11:15-16; Matt 3:16-17; John 1:32; Luke 3:22; Is 44:3",
    ];

    // Converts the compound string of references into a string vector of integer codes
    for test_ref in test_refs {
        let final_refs = Parsing::convert_refs(test_ref);
        println!("{:?}", test_ref);
        println!("{:?}", final_refs);
        println!("");
    }

}
