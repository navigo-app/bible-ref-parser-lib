mod bible_map;
mod title_hashmap;

use crate::bible_map::BibleMap;

use regex::Regex;
use std::vec;
use std::collections::HashMap;

fn main() {
    let test_refs: Vec<&str> = vec![
        // "Bad 6:66, John 3:16",
        // "Gen 50:1-Exo 2:1;Psa 5:6,7",
        // "Jn 3:16-17, Ge 1:1 - Ex 2:1, Lev 9:12",
        // "2 Peter 1-1 John 2, Romans 5:12, 2 Corinthians 11:12-Galatians 1:5",
        // "Genesis 1:1, 2:3",
        // "Genesis 1:1",
        // "Psalm 119:150",
        // "Revelation 22:1",
        // "2 John 1:3",
        // "Gen 1",
        // "Gen 1;2,3",
        // "Gen 1:1,2,4",
        // "Gen 1:1-2:20",
        // "Gen 1:1-2:20,23",
        "Gen 1; Psalm 1",
        // "Gen 1 & Psalm 1",
        // "Gen 1 and Psalm 1",
        // "Gen 7:21,19:17,20-27",
        // "Mark 1,2,3:27",
        // "Gen 1:1-20; 24",
        // "Gen 1; 24",
        // "Psalm 6:22, 1 John 3:15, Gen 1:1",
        // "GENESIS 7:21,19:17,20-27;1 John 1,2,3:27,Psalm 1,2:7",
    ];
println!("{:?}", test_refs[0]);
    let mut final_refs: Vec<String> = Vec::new();

    // Uppercase all refs. Remove any spaces in numbered-books (e.g. change '2 Peter' to '2PETER')
    let vec_refs: Vec<String> = test_refs
        .iter()
        .map(|r| r.to_uppercase())
        .map(|r| BibleMap::fix_numbered_book(r))
        .map(|r| r.replace(" ", ""))
        .map(|r| r.replace("AND", "&"))
        .collect();
       
    // span_refs gathers the refs that span two books, e.g. Gen 50-Exo 1
    let mut span_refs: Vec<String> = Vec::new();
    // other_refs gathers all other refs
    let mut other_refs: Vec<String> = Vec::new();

    let book_span_re = Regex::new(r"\b\d{0,1}[A-Z]+.*-\d{0,1}[A-Z]+.*\b").unwrap();
    for r in vec_refs.iter() {
        if !book_span_re.is_match(r) {
            other_refs.push(r.to_string());
        } else {
            let split_rs: Vec<&str> = r.split(|c| c == ',' || c == ';').collect();
            for split_r in split_rs.iter() {
                if book_span_re.is_match(split_r) {
                    span_refs.push(split_r.to_string());
                    let o_ref = r
                        .replace(split_r,"")
                        .trim_start_matches(|c| c == ',' || c == ';')
                        .trim_end_matches(|c| c == ',' || c == ';')
                        .to_string();
                    if !book_span_re.is_match(o_ref.as_str())
                        && !other_refs.contains(&o_ref) {
                        other_refs.push(o_ref);
                    }
                } else {
                    let o_ref = split_r.to_string();
                    if !other_refs.contains(&o_ref) {
                        other_refs.push(o_ref);
                    }
                }
            }
        }
    }
// println!("span_refs = {:?}, other_refs = {:?}", span_refs, other_refs);

    for other_ref in other_refs.iter_mut() {
        let result = parse_compound_ref(other_ref);
        let ref_books = result.0;
        let ref_chapter_verses = result.1;

        let parsed_chapter_verses: Vec<Vec<String>> = ref_chapter_verses
            .iter()
            .flat_map(|r| vec![iterative_parse(vec![r.to_string()])])
            .collect();

        // Verify that the number of strings in ref_books equals the number of Vec<String> in parsed_chapter_verses
        if ref_books.len() != parsed_chapter_verses.len() {
            panic!("Unequal number of ref_books ({}) and parsed_chapter_verses ({})",
                ref_books.len(),
                parsed_chapter_verses.len()
            );
        }

        // iterate over ref_books and parsed_chapter_verses simultaneously
        let book_chapter_verses: Vec<String> = ref_books.into_iter().rev()
            .zip(parsed_chapter_verses.into_iter().rev())
            .flat_map(|(book, verses)| {
                // For each tuple, iterate over the Vec<String> in parsed_chapter_verses
                verses.into_iter()
                    .map(move |verse| format!("{}{}", book, verse))
            })
            .collect::<Vec<String>>();

        let code_bcv_refs: Vec<String> = book_chapter_verses
            .iter()
            .map(|bcv_ref| bcv_ref_to_id_code(bcv_ref))
            .collect::<Result<Vec<String>, _>>()
            .unwrap_or_else(|err| {
                println!("Error: {}", err);
                Vec::new()
            });

        for c in code_bcv_refs.iter() {
            if !final_refs.contains(c) {
                final_refs.push(c.to_string());
            }
        }
    }

    let titles:Vec<String> = title_hashmap::create_vector();
    let my_map: HashMap<&str, &str> = title_hashmap::create_hashmap();

    for span_ref in span_refs {
        let code_span_ref = span_ref_to_id_code(&span_ref, &titles, &my_map);
        if !final_refs.contains(&code_span_ref) {
            final_refs.push(code_span_ref);
        }
    }

    for f in final_refs.iter() {
        println!("{f}");
    }
    
}

/*  Takes any book, chapter, verses reference and returns the id_code. E.g., it converts "GEN1" into 1001001-1001999,
    or "GE1:2" into 1001002, or "GEN1:2-3" into 1001002-1001003, or "GEN1:2-3:40" into 1001002-1003040
*/
fn bcv_ref_to_id_code(bcv_ref: &str) -> Result<String, &'static str> {
    // capture the book context (now always the first three characters)
    let (book, remainder) = bcv_ref.split_at(3);
    let chapter_verse: Vec<&str> = remainder.split(':').collect();
    let book_id = match BibleMap::get_book_id_by_code(book) {
        Some(id) => id,
        None => return Err("Invalid book code"),
    };

    if chapter_verse.len() == 1 {
        // single chapter; e.g., "GEN1"
        let chapter = chapter_verse[0];
        Ok(format!("{}{:03}001-{}{:03}999",
            book_id,
            chapter.parse::<u32>().unwrap_or(0),
            book_id, chapter.parse::<u32>().unwrap_or(0)
        ))
    } else if chapter_verse.len() == 2 {
        let chapter = chapter_verse[0];
        let verses = chapter_verse[1];
        if !verses.contains('-') {
            // single chapter and verse; e.g., "GEN1:2"
            Ok(format!("{}{:03}{:03}",
                book_id,
                chapter.parse::<u32>().unwrap_or(0),
                verses.parse::<u32>().unwrap_or(0)
            ))
        } else {
            // single chapter with multiple verses; e.g. "GEN1:2-3"
            let parts: Vec<&str> = verses.split('-').collect();
            let first_verse = parts[0];
            let second_verse = parts[1];
            Ok(format!("{}{:03}{:03}-{}{:03}{:03}",
                book_id,
                chapter.parse::<u32>().unwrap_or(0),
                first_verse.parse::<u32>().unwrap_or(0),
                book_id, chapter.parse::<u32>().unwrap_or(0),
                second_verse.parse::<u32>().unwrap_or(0)
            ))
        }
    } else if chapter_verse.len() == 3 {
        // multiple chapter and verse; e.g. "GEN1:2-3:40"
        let parts: Vec<&str> = remainder.split('-').collect();
        let first_chapter_verse = parts[0];
        let second_chapter_verse = parts[1];
        Ok(format!("{}{:03}{:03}-{}{:03}{:03}",
            book_id,
            first_chapter_verse.split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap_or(0),
            first_chapter_verse.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap_or(0),
            book_id,
            second_chapter_verse.split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap_or(0),
            second_chapter_verse.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap_or(0)
        ))
    } else {
        // Default case, return an error
        Err("Invalid input format")
    }
}

/*  These functions take any reference that matches the spanning of two books and returns the id_code.
    E.g., they convert "GE1:1-EX2:1" into 1001001-2002001, or "2PETER1-1JOHN2" into 61001001-62002999
*/
fn span_ref_to_id_code(span_ref: &str, titles: &[String], my_map: &HashMap<&str, &str>) -> String {
    let span_ref_split: Vec<&str> = span_ref.split('-').collect();
    let mut id_code0 = String::new();
    let mut id_code1 = String::new();

    for (idx, sr) in span_ref_split.iter().enumerate() {
        for title in titles {
            if sr.contains(title) {
                let abbr_title = my_map.get(title.as_str()).ok_or("Error: No value").unwrap();
                let verses: String = sr.trim_start_matches(title).to_string();
                let chapter_verse: Vec<&str> = verses.split(':').collect();
                let chapter = chapter_verse[0];
                let verses = chapter_verse.get(1).unwrap_or(&"");
                let book_id = BibleMap::get_book_id_by_name(abbr_title);

                match book_id {
                    Some(id) => {
                        let id_code = if verses.is_empty() {
                            format!("{}{:03}{}", id, chapter.parse::<u32>().unwrap_or(0), if idx == 0 { "001" } else { "999" })
                        } else {
                            format!("{}{:03}{:03}", id, chapter.parse::<u32>().unwrap_or(0), verses.parse::<u32>().unwrap_or(0))
                        };
                        if idx == 0 {
                            id_code0 = id_code;
                        } else {
                            id_code1 = format!("-{}", id_code);
                        }
                    },
                    _ => println!("Book not found"),
                }
                break;
            }
        }
    }

    if !id_code0.is_empty() && !id_code1.is_empty() {
        let num_id_code0 = extract_digits(&id_code0);
        let num_id_code1 = extract_digits(&id_code1);

        if num_id_code0 >= num_id_code1 {
            println!("Error: id_code0 should be less than id_code1");
        }
    }

    format!("{}{}", id_code0, id_code1)
}

fn extract_digits(s: &str) -> u32 {
    s.chars().filter(|&c| c.is_digit(10))
             .collect::<String>()
             .parse::<u32>()
             .unwrap_or(0)
}

/*  Separates the compound reference into a tuple containing a vector of book strings
    and a vector of verse strings, in the same order
*/
fn parse_compound_ref(input: &str) -> (Vec<String>, Vec<String>) {
    // the book_re regex matches any Bible book
    let book_pattern = r"(\d{0,1}[A-Z]+.*?)";
    let book_re = Regex::new(book_pattern).unwrap();

    // book_matches is a vector of the Bible book titles in the input
    let book_matches: Vec<String> = book_re
        .captures_iter(input)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect();

    let book_match_vec = book_matches.clone();

    // m_len is the number of distinct books in the input
    let m_len = book_match_vec.len();

    let mut temp_input = input;

    // book_vec will be the vector of books in the input reference
    let mut book_vec: Vec<String> = Vec::new();
 
    // verses_vec will be the vector of chapter & verses in the input
    let mut verses_vec: Vec<String> = Vec::new();

    // iterate through the number of books in the input to get the books and chapter & verses
    for i in 0..m_len {
        // m is the Option<book>s in the input (in reverse order)
        let m = book_match_vec.get(m_len - i - 1);
        // index is the Option<index> of the start of the book title (in reverse order)
        let index = m.map(|val| temp_input.find(val).unwrap_or(temp_input.len()));
        // left and right are the input divided left and right at the indexes
        let (left, right) = temp_input.split_at(index.unwrap());

        let mut stripped_right = "";
        if let Some(val) = m {

            if let Some(stripped) = right.strip_prefix(val) {
                stripped_right = stripped;
            }

            let bookcode = BibleMap::get_book_code_by_name(val);
            if bookcode == None {
                eprintln!("{} is not recognized", val);
                break;
            }

            if let Some(code) = bookcode {
                book_vec.push(code.to_string());
            }
        }

        // Remove punctuation from the end of the "right" string
        let right_no_punct = stripped_right.trim_end_matches(|c: char| c.is_ascii_punctuation());
        verses_vec.push(right_no_punct.to_string());
 
        temp_input = left;
    }

    (book_vec, verses_vec)

}

//  Parses the verses in a compound chapter-verse string into a vector of verses
fn parse_chapter_verse(input: String) -> Vec<String> {
    // always parse on a semicolon
    if input.contains(';') {
        return input.split(';').map(|c| c.to_string()).collect();
    }
    // if the compound reference contains only chapters, parse on a comma
    else if !input.contains(':') {
        return input.split(',').map(|c| c.to_string()).collect();
    }
    /* if any compound ref contains commas
        1. if the first ref contains a colon, all refs after with no colon get the colon chapter attached
        2. otherwise, simply parse on all commas
    */
    else if input.contains(',') {
        return parse_comma_verses(input);
    }
    // otherwise, pass the compound reference as a string vector
    else {
        return vec![input];
    }
}

// Checks to see the parse_chapter_verse has any effect, and if so, runs the function again
fn iterative_parse(input: Vec<String>) -> Vec<String> {
    let mut output = input;
    let mut old_output;
 
    loop {
        old_output = output.clone();
 
        output = output
            .into_iter()
            .flat_map(parse_chapter_verse)
            .collect();
 
        if old_output == output {
            break;
        }
    }

    output
}

fn parse_comma_verses(input: String) -> Vec<String> {
    let mut chapter_verse_vec = Vec::new();
  
    let comma_split: Vec<&str> = input.split(',').collect();
    let first_part = comma_split[0];

    if first_part.contains(':') {
    // parse chapter and verses
        chapter_verse_vec.push(first_part.to_string());
        let chapter_verse_split: Vec<&str> = first_part.split(':').collect();
        let chapter = chapter_verse_split[0];
        let verses = &comma_split[1..];

        for verse in verses {
            // the reference contains only one chapter:verse
            if !verse.contains(':') {
                // spans a chapter, so the chapter:verse must use the 2nd chapter
                if first_part.contains('-') {
                    let second_chapter_verse = first_part
                        .split('-')
                        .collect::<Vec<&str>>()[1];

                    let second_chapter = second_chapter_verse
                        .split(':')
                        .collect::<Vec<&str>>()[0];

                    chapter_verse_vec.push(format!("{}:{}", second_chapter, verse));
                } else {
                    chapter_verse_vec.push(format!("{}:{}", chapter, verse));
                }
            } else {
            // recursively call the function until the reference contain only one chapter:verse
                let mut remaining_verses = String::new();
                let second_part: String = verses.join(",");

                let elements: Vec<&str> = second_part.split(',').collect();
                if let Some(index) = elements.iter().position(|&x| x.contains(':')) {
                    let result: Vec<&str> = elements[index..].to_vec();
                    remaining_verses = result.join(",");
                }
                for element in parse_comma_verses(remaining_verses) {
                    chapter_verse_vec.push(element);
                }
                break;
            }
        }
    } else {
    // parse chapter only
        for verse in &comma_split {
            chapter_verse_vec.push(verse.to_string());
        }
    }
  
    chapter_verse_vec
}
