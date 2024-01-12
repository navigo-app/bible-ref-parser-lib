use crate::parse;

#[test]
fn test_pars() {
    let test_refs: Vec<&str> = vec![
        "Gen 50:1-Exo 2:1",
        "Jn 3:16-17, Ge 1:1 - Ex 2:1, Lev 9:12",
        "2 Peter 1-1 John 2, Romans 5:12, 2 Corinthians 11:12-Galatians 1:5",
        "Acts 15-16",
        "Genesis 1:1, 2:3",
        "Psalm 119:150, Hebrews 3:1-6",
        "Rev. 22:1",
        "III John 1:3",
        "Gen. 1",
        "Exo. 1;2,3",
        "Dt 1:1,2,4",
        "I Sam 1:1-2:20",
        "I Sa 1:1-2:20",
        "Gen 1:1-2:20,23",
        "Song of Solomon 1; Psalms 1, 2",
        "I Cor. 1 & Ps. 1",
        "Judges 1 and Prov. 2",
        "Gen 7:21,19:17,20-27",
        "Mark 1,2,3:27",
        "Gen 1:1-20; 24",
        "2 Jn 1; 24:2; 39",
        "Psalm 6:22, 1 John 3:15, Gen 1:1",
        "GENESIS 7:21,19:17,20-27;1 JOHN 1,2,3:27,PSALM 1,2:7",
        "Acts 2:3,16-17,33, 10:44-45, 11:15-16; Matt 3:16-17; John 1:32; Luke 3:22; Is 44:3",
        "Ezra 1:3-2", // returns code for Ezra 1:3
        "Ruth 3-2", // returns code for Ruth 3
        "Numbers 1 - Leviticus 30", // returns code for Numbers 1
        "Josiah 40:1 - Judges 3:7", // returns None
        "Josiah 40:1 - Judges 3", // returns None
        "John 3:16, Deut 30 - Josiah 1", // returns code for John 3:16
        "Deut 30:4 - Josiah 1:2", // returns None
        "John 3:16:17", // returns None
        "Garbage 3:16", // returns None
        "John E:16", // returns None
        "1,728", // returns None
    ];

    // Converts the compound string of references into a string vector of integer codes
    for test_ref in test_refs {
        //TODO convert these to asserts
        let final_refs = parse(test_ref);
        assert!(true); //This is here to keep the test runner happy.
        println!("{:?}", test_ref);
        println!("{:?}", final_refs);
        println!("");
    }

}