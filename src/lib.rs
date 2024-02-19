mod tests;
pub mod utils;

use std::collections::HashMap;
use regex::Regex;
use crate::utils::title_hashmap::TitleMap;
use crate::utils::bible_map::BibleMap;

pub fn parse(test_ref: &str) -> Option<Vec<String>> {
    let vec_ref: String = clean_ref(test_ref);

    let titles: Vec<String> = TitleMap::create_vector();
    let my_map: HashMap<&str, &str> = TitleMap::create_hashmap();

    // references that span books; e.g., ["Gen 50:1-Exo 2"]
    let mut span_refs: Vec<String> = Vec::new();
    // regular (non-spanning) references; e.g., ["Psalm 119:150"]
    let mut reg_refs: Vec<String> = Vec::new();
    // aggregation of integer codes; e.g. ["19119150,58003001-58003006"]
    let mut agg_refs: Vec<String> = Vec::new();

    let book_span_re = Regex::new(r"\b\d{0,1}[A-Z]+.*-\d{0,1}[A-Z]+.*\b").unwrap();

    if !book_span_re.is_match(vec_ref.as_str()) {
        agg_refs.push(process_bcv_ref(&vec_ref));
        reg_refs.push(vec_ref);
    } else {
        let split_rs: Vec<&str> = vec_ref.split(|c| c == ',' || c == ';').collect();
        for split_r in split_rs.iter() {
            if book_span_re.is_match(split_r) {
                agg_refs.push(span_ref_to_id_code(split_r, &titles, &my_map));
                span_refs.push(split_r.to_string());
                let o_ref = vec_ref
                    .replace(split_r, "")
                    .trim_start_matches(|c| c == ',' || c == ';')
                    .trim_end_matches(|c| c == ',' || c == ';')
                    .to_string();

                if !book_span_re.is_match(o_ref.as_str())
                    && !reg_refs.contains(&o_ref) {
                    agg_refs.push(process_bcv_ref(&o_ref));
                }
            } else {
                let o_ref = split_r.to_string();
                if !reg_refs.contains(&o_ref) {
                    agg_refs.push(process_bcv_ref(&o_ref));
                }
            }
        }
    }

    let mut final_refs: Vec<String> = Vec::new();

    for f in agg_refs.iter() {
        let f_refs: Vec<String> = f.split(',').map(|r| r.to_string()).collect();
        for f_ref in f_refs.iter() {
            if !f_ref.is_empty() && !final_refs.contains(f_ref) {
                final_refs.push(f_ref.to_string());
            }
        }
    }

    // check for common problems
    let filtered_refs: Vec<_> = final_refs
        .iter()
        .filter_map(|final_ref| {
            if final_ref.contains('-') {
                let f_split: Vec<&str> = final_ref.split('-').collect();
                let code1 = extract_digits(f_split[0]);
                let code2 = extract_digits(f_split[1]);

                if code1 == 0 || code2 == 0 {
                    return None;
                } else if is_chapter_value_zero(code1) || is_chapter_value_zero(code2) {
                    return None;
                } else if code1 > code2 {
                    if ends_in_999(code2) {
                        return Some(vec![format!("{}-{}", code1, code1 + 998)]);
                    } else {
                        return Some(vec![format!("{}", code1)]);
                    }
                } else {
                    return Some(vec![final_ref.clone()]);
                }
            } else {
                return Some(vec![final_ref.clone()]);
            }
        })
        .collect();

    if filtered_refs.is_empty() {
        None
    } else {
        Some(filtered_refs.concat())
    }
}

// Clean reference into a standard form
pub fn clean_ref(test_ref: &str) -> String {
    // remove a or b after verse number
    let pattern = Regex::new(r"(\d)[ab]{1}").unwrap();
    let return_ref = pattern.replace_all(test_ref, |caps: &regex::Captures| {
        caps[1].to_string()
    });

    return_ref
        .to_uppercase()
        .replace(" ", "")
        .replace(".", ":") // for chapter:verse; book punctuation gets stripped later
        .replace("AND", "&")
}

// Convert book chapter:verses into integer code string
fn process_bcv_ref(other_ref: &String) -> String {
    let result = parse_compound_ref(&other_ref);
    let ref_books = result.0;
    let ref_chapter_verses = result.1;

    let parsed_chapter_verses: Vec<Vec<String>> = ref_chapter_verses
        .iter()
        .flat_map(|r| vec![iterative_parse(vec![r.to_string()])])
        .collect();

    // iterate over ref_books and parsed_chapter_verses simultaneously
    let book_chapter_verses: Vec<String> = ref_books.into_iter().rev()
        .zip(parsed_chapter_verses.into_iter().rev())
        .flat_map(|(book, verses)| {
            // For each tuple, iterate over the Vec<String> in parsed_chapter_verses
            verses.into_iter()
                .map(move |verse| format!("{}{}", book, verse))
        })
        .collect::<Vec<String>>();

    let temp_code_bcv_refs: Vec<String> = book_chapter_verses
        .iter()
        .map(|bcv_ref| bcv_ref_to_id_code(bcv_ref))
        .collect::<Result<Vec<String>, _>>()
        .unwrap_or_else(|err| {
            println!("Error: {}", err);
            Vec::new()
        });

    temp_code_bcv_refs.join(",")
}

//  Separates the compound reference into a tuple containing a vector of book strings
//  and a vector of verse strings, in the same order
pub fn parse_compound_ref(input: &str) -> (Vec<String>, Vec<String>) {
    // matches any Bible book
    let book_pattern = r"(\d{0,1}[A-Z]+.*?)";
    let book_re = Regex::new(book_pattern).unwrap();

    // book_matches is a vector of the Bible book titles in the input
    let book_matches: Vec<String> = book_re
        .captures_iter(input)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
        .collect();

    let book_match_vec = book_matches.clone();

    // book_num is the number of distinct books in the input
    let book_num = book_match_vec.len();

    let mut temp_input = input;

    // book_vec will be the vector of books in the input reference
    let mut book_vec: Vec<String> = Vec::new();

    // verses_vec will be the vector of chapter & verses in the input
    let mut verses_vec: Vec<String> = Vec::new();

    // iterate through the number of books in the input to get the books and chapter & verses
    for i in 0..book_num {
        // ref_book is the Option<book>s in the input (in reverse order)
        let ref_book: Option<&String> = book_match_vec.get(book_num - i - 1);
        // index is the Option<index> of the start of the book title (in reverse order)
        let index = ref_book.map(|val| temp_input.rfind(val).unwrap_or(temp_input.len()));
        // left and right are the input divided left and right at the indexes
        let (left, right) = temp_input.split_at(index.unwrap());

        let mut stripped_right = String::new();
        if let Some(book_val) = ref_book {
            if let Some(stripped) = right.strip_prefix(book_val) {
                stripped_right = stripped
                    .trim_end_matches(|c: char| c.is_ascii_punctuation())
                    .trim_start_matches(|c: char| c.is_ascii_punctuation())
                    .to_string();
            }

            let bookcode = BibleMap::get_book_code_by_name(book_val);
            if bookcode == None {
                break;
            }

            if let Some(code) = bookcode {
                book_vec.push(code.to_string());
            }
        }

        verses_vec.push(stripped_right);

        temp_input = left;
    }

    (book_vec, verses_vec)

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

//  Takes any book, chapter, verses reference and returns the id_code. E.g., it converts "GEN1" into 1001001-1001999,
//  or "GEN1:2" into 1001002, or "GEN1:2-3" into 1001002-1001003, or "GEN1:2-3:40" into 1001002-1003040,
//  or "GEN" into 1001001-1999999
fn bcv_ref_to_id_code(bcv_ref: &str) -> Result<String, &'static str> {
    // capture the book context (now always the first three characters)
    let (book, remainder) = bcv_ref.split_at(3);
    let chapter_verse: Vec<&str> = remainder.split(':').collect();
    let book_id = match BibleMap::get_book_id_by_code(book) {
        Some(id) => id,
        None => return Err("Invalid book code"),
    };

    if chapter_verse.len() == 1 {
        // single chapter or span of chapters
        let s_chapter = chapter_verse[0];

        // book without chapter or verses
        if s_chapter.is_empty() {
            Ok(format!("{}001001-{}999999",
                        book_id,
                        book_id
            ))
        // book and multiple chapters
        } else if s_chapter.contains('-') {
            let split: Vec<&str> = s_chapter.split('-').collect();
            let chapter1 = split[0].parse::<u32>().unwrap();
            let chapter2 = split[1].parse::<u32>().unwrap();

            if chapter1 > 999 || chapter2 > 999 {
                Err("Chapter value too high")
            } else {
                Ok(format!("{}{:03}001-{}{:03}999",
                            book_id,
                            chapter1,
                            book_id,
                            chapter2
                ))
            }
        // book and a single chapter
        } else {
            let mut schapter:u32 = 0;
            match s_chapter.parse::<u32>() {
                Ok(_) => {
                    schapter = s_chapter.parse::<u32>().unwrap();
                }
                Err(_) => {
                    println!("The chapter number is not numeric.");
                }
            }

            if schapter > 999 {
                Err("Chapter value too high")
            } else {
                Ok(format!("{}{:03}001-{}{:03}999",
                            book_id,
                            schapter,
                            book_id,
                            schapter
                ))
            }
        }
    } else if chapter_verse.len() == 2 {
        let chapter = chapter_verse[0].parse::<u32>().unwrap();
        let verses = chapter_verse[1];
        if !verses.contains('-') {
            // single chapter and verse; e.g., "GEN1:2"
            let sverses = verses.parse::<u32>().unwrap();

            if chapter > 999 {
                Err("Chapter value too high")
            } else if sverses > 999 {
                Err("Verse value too high")
            } else {
                Ok(format!("{}{:03}{:03}",
                            book_id,
                            chapter,
                            sverses
                ))
            }
        } else {
            // single chapter with multiple verses; e.g. "GEN1:2-3"
            let parts: Vec<&str> = verses.split('-').collect();
            let first_verse = parts[0].parse::<u32>().unwrap();
            let second_verse = parts[1].parse::<u32>().unwrap();

            if chapter > 999 {
                Err("Chapter value too high")
            } else if first_verse > 999 || second_verse > 999 {
                Err("Verse value too high")
            } else {
                Ok(format!("{}{:03}{:03}-{}{:03}{:03}",
                            book_id,
                            chapter,
                            first_verse,
                            book_id,
                            chapter,
                            second_verse
                ))
            }
        }
    } else if chapter_verse.len() == 3 {
        // multiple chapter and verse; e.g. "GEN1:2-3:40"
        let parts: Vec<&str> = remainder.split('-').collect();
        let first_chapter = parts[0].split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let first_verse = parts[0].split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let second_chapter_verse = match parts.get(1) {
            Some(verse) => verse,
            None => return Err("Invalid reference")
        };
        let second_chapter = second_chapter_verse.split(':').collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let second_verse = second_chapter_verse.split(':').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();

        if first_chapter > 999 || second_chapter > 999 {
            Err("Chapter value too high")
        } else if first_verse > 999 || second_verse > 999 {
            Err("Verse value too high")
        } else {
            Ok(format!("{}{:03}{:03}-{}{:03}{:03}",
                        book_id,
                        first_chapter,
                        first_verse,
                        book_id,
                        second_chapter,
                        second_verse
            ))
        }
    } else {
        // Default case, return an error
        Err("Invalid input format")
    }
}

//  Parses the verses in a compound chapter:verse string into a vector of verses
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

// Parses chapter and verses (e.g. "1:2,3") into a vector of chapter:verse strings (e.g. ["1:2", "1:3"])
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
                // first part spans a chapter or spans multiple verses
                if first_part.contains('-') {
                    let second_chapter_verse = first_part
                        .split('-')
                        .collect::<Vec<&str>>()[1];
                    // the second part contains a chapter, use it
                    if second_chapter_verse.contains(':') {
                        let second_chapter = second_chapter_verse
                            .split(':')
                            .collect::<Vec<&str>>()[0];
                        chapter_verse_vec.push(format!("{}:{}", second_chapter, verse));
                        // use the chapter from the first part
                    } else {
                        chapter_verse_vec.push(format!("{}:{}", chapter, verse));
                    }
                    // first part has only chapter and verse
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

//  Takes any reference that matches the spanning of two books and returns the id_code.
//  E.g., they convert "GE1:1-EX2:1" into 1001001-2002001, or "2PETER1-1JOHN2" into 61001001-62002999
fn span_ref_to_id_code(span_ref: &str, titles: &Vec<String>, my_map: &HashMap<&str, &str>) -> String {
    let span_ref_split: Vec<&str> = span_ref.split('-').collect();
    let mut id_code0 = String::new();
    let mut id_code1 = String::new();

    for (idx, sr) in span_ref_split.iter().enumerate() {
        for title in titles.iter() {
            if sr.contains(title) {
                let abbr_title = my_map.get(title.as_str()).unwrap();
                let ref_minus_title: String = sr.trim_start_matches(title).to_string();

                if ref_minus_title.chars().any(char::is_alphabetic) {
                    // the title contained but never equals a recognized title, so return blank
                    eprintln!("Book not found");
                    return String::new();
                }

                let chapter_verse: Vec<&str> = ref_minus_title.split(|c| c == ':' || c == '.').collect();
                let chapter = chapter_verse[0].parse::<u32>().unwrap_or(0);
                let verses = chapter_verse.get(1).unwrap_or(&"");
                let book_id = BibleMap::get_book_id_by_name(abbr_title);

                if chapter > 999 {
                    eprintln!("Chapter value too high");
                    return String::new();
                }

                match book_id {
                    Some(id) => {
                        let id_code = if verses.is_empty() {
                            if chapter == 0 {
                                format!("{}{:03}{}",
                                         id,
                                         if idx == 0 { "001" } else { "999" },
                                         if idx == 0 { "001" } else { "999" })
                            } else {
                                format!("{}{:03}{}",
                                         id,
                                         chapter,
                                         if idx == 0 { "001" } else { "999" })
                            }
                        } else {
                            let sverses = verses.parse::<u32>().unwrap();
                            if sverses > 999 {
                                eprintln!("Verse value too high");
                                return String::new();
                            }
                            format!("{}{:03}{:03}",
                                     id,
                                     chapter,
                                     sverses)
                        };
                        if idx == 0 {
                            id_code0 = id_code;
                        } else {
                            id_code1 = format!("-{}", id_code);
                        }
                    },
                    _ => eprintln!("Book not found"),
                }
                break;
            }
        }
    }

    format!("{}{}", id_code0, id_code1)
}

// Used in span_ref_to_id_code
fn extract_digits(s: &str) -> u32 {
    s.chars().filter(|&c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap_or(0)
}

fn is_chapter_value_zero(num: u32) -> bool {
    let div_mil = num / 1000000;
    let diff = num - div_mil * 1000000;
    diff < 1000
}

fn ends_in_999(num: u32) -> bool {
    let div_k = num / 1000;
    let diff = num - div_k * 1000;
    diff == 999
}