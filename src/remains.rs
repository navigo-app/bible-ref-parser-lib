// mod bible_map;

// use crate::bible_map::BibleMap;
// use regex::Regex;

// fn main() {
//     let test_refs = vec![
//         "Genesis 1:1",
//         "Psalm 119:150",
//         "Revelation 22:1",
//         "2 John 1:3",
//         "Gen 1",
//         "Gen 1:1,2,4",
//         "Gen 1:1-2:20",
//         "Gen 1:1-2:20,23",
//         "Gen 1:1-Exo 2:1",
//         "Gen 1:1 - Exo 2:1",
//         "Gen 1-Exo 2",
//         "Gen 1; Psalm 1",
//         "Gen 1 & Psalm 1",
//         "Gen 1 and Psalm 1",
//         "Gen 7:21,19:17,20-27",
//         "Mark 1,2,3:27",
//         "Gen 1:1-20; 24",
//         "Gen 1; 24",
//         "Psalm 6:22, 1 John 3:15"
//     ];

//     let id_ref = vec![
//         "1001001",
//         "19119150",
//         "66022001",
//         "63001003",
//         "1001001-1001999",
//         "1001001,1001002,1001004",
//         "1001001-1002020",
//         "1001001-1002020,1002023",
//         "1001001-2002001",
//         "1001001-2002001",
//         "1001001-2002999",
//         "1001001-1001999,19001001-19001999",
//         "1001001-1001999,19001001-19001999",
//         "1001001-1001999,19001001-19001999",
//         "1007021,1019017,1019020-1019027",
//         "41001001-41001999,41002001-41002999,41003027",
//         "1001001-1001020,1001024",
//         "1001001-1001999,1024001-1024999",
//         "19006022,62003015"
//     ];
    
//     let mut new_id_ref: Vec<String> = Vec::new();

//     for _ref in test_refs {
//         let refs = BibleMap::fix_numbered_book(_ref);
//         let parts: Vec<&str> = refs.split(' ').collect();

//         let book = parts[0];
//         let chapter_verse: Vec<&str> = parts[1].split(':').collect();
//         let chapter = chapter_verse[0];
//         let verses = chapter_verse.get(1).unwrap_or(&"");
//         let book_id = BibleMap::get_book_id_by_name(book);

//         match book_id {
//             Some(id) => {
//                 if verses.len() == 0 {
//                     let code_id = format!("{}{:03}001-{}{:03}999", id, chapter.parse::<u32>().unwrap_or(0), id, chapter.parse::<u32>().unwrap_or(0));
//                     new_id_ref.push(code_id);
//                 } else {
//                     let code_id = format!("{}{:03}{:03}", id, chapter.parse::<u32>().unwrap_or(0), verses.parse::<u32>().unwrap_or(0));
//                     new_id_ref.push(code_id);
//                 }
//             },
//             _ => println!("Book not found"),
//         }
//     }

//     for (id, new_id) in id_ref.iter().zip(new_id_ref.iter()) {
//         if id == new_id {
//             println!("{}  {}", id, new_id);
//         } else {
//             println!("{}  \x1b[31m{}\x1b[0m", id, new_id);
//         }
//     }

    // let book_code = BibleMap::get_book_code_by_name("1 John");
    // match book_code {
    //     Some(code) => println!("The book code is {}", code),
    //     None => println!("Book not found"),
    // }

    // let book_id = BibleMap::get_book_id_by_name("genesis");
    // match book_id {
    //     Some(id) => println!("The book id is {}", id),
    //     None => println!("Book not found"),
    // }

    // let book_id = BibleMap::getname_book_id_by_code("1JN");
    // match book_id {
    //     Some(id) => println!("The book id is {}", id),
    //     None => println!("Book not found"),
    // }

}

// for r in vec_refs {
//     if r.is_empty() {
//         break;
//     }
//     let (book_context, remainder) = r.split_at(3);
//     let re = Regex::new(r"[:,;-]").unwrap();
//     let parts: Vec<&str> = re.split(remainder).collect();
//     let chapter_context: &str = parts[0];
//     let remainder = remainder.strip_prefix(chapter_context).unwrap_or(remainder);
//     println!("{:?} -- {}, {}, {:?}", r, book_context, chapter_context, remainder);
// }

// fn main() {
//     let titles = vec!["Genesis", "Exodus", "Psalm", "Gen", "Exo", "Ps"]
//         .iter()
//         .map(|s| s.to_string())
//         .collect::<Vec<String>>();

//     let other_ref = "Genesis1:1,Ps5:1,Exo5:3-7,17".to_string();

//     let mut found_titles: Vec<String> = Vec::new();

//     for title in &titles {
//         if other_ref.contains(title) && !found_titles.iter().any(|found_title| found_title.contains(title)) {
//             found_titles.push(title.clone());
//         }
//     }
//     println!("{:?}", found_titles);
// }

// let mut input = "GENESIS7:21,19:17,20-27;1JOHN1,2,3:27,PSALM1,2:7";
// let pattern = r"(\d{0,1}[A-Z]+.*?)";
// let re = Regex::new(pattern).unwrap();

// let mut final_vec: Vec<&str> = Vec::new();
// let matches: Vec<String> = re
// .captures_iter(input)
// .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_string()))
// .collect();

// let match_vec = matches.clone();
// let m_len = match_vec.len();

// for i in (0..m_len) {   
//     let m = match_vec.get(m_len-i-1);
//     let index = m.map(|val| input.find(val).unwrap_or(input.len()));
//     let (left, right) = input.split_at(index.unwrap());

//     let mut vec = Vec::new();
//     vec.push(left);
//     if let Some(val) = m {
//         vec.push(val);
//     }
//     final_vec.push(right);

//     input = left;
// }

// println!("{:?}", final_vec);

//     // Upercase all refs. Remove any spaces in numbered-books (e.g. change '2 Peter' to '2PETER')
//     let vec_refs: Vec<String> = test_refs
//         .iter()
//         .map(|r| r.to_uppercase())
//         .map(|s| BibleMap::fix_numbered_book(s))
//         .collect();
       
//     // this vector gathers the refs that span two books, e.g. Gen 50-Exo 1
//     let mut span_refs = Vec::new();
//     // this vector gathers all other refs
//     let mut other_refs = Vec::new();
    
//     let re = Regex::new(r"\b[1|2|3]{0,1}\s*[A-Z]+\s*[0-9\:]+\s*-\s*[1|2|3]{0,1}\s*[A-Z]+\s*[0-9\:]+\b").unwrap();
//     for r in vec_refs.iter() {
//         if re.is_match(r) {
//             span_refs.push(r.clone());
//         } else {
//             other_refs.push(r.clone());
//         }
//     }
        
//     // format the spanned refs with book code and remove spaces
//     let new_span_refs: Vec<String> = span_refs
//         .iter()
//         .map(|s| s.replace("-", " - "))
//         // separate each string reference in the vector by % (this is undone below)
//         .map(|s| {
//             let mut s = s;
//             s.push_str(" %");
//             s
//         })
//         .flat_map(|t| {
//             t.split_whitespace()
//             .map(|word| {
//                 if word.chars().any(char::is_alphabetic) {
//                     BibleMap::get_book_code_by_name(word).map(String::from)
//                 } else {
//                     Some(word.to_string())
//                 }
//             }).collect::<Vec<Option<String>>>()
//         })
//         // remove the Option and recollect
//         .filter_map(|opt| opt)
//         .collect::<Vec<String>>()
//         // collect references between %
//         .join(" ")
//         .split('%')
//         .map(|s| s.replace(" ", ""))
//         .map(String::from)
//         .filter(|s| !s.is_empty())
//         .collect();

//     // println!("{:?}", new_span_refs);

//     println!("{:?}", other_refs);
//     // format the other refs with book code and remove spaces
//     static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\b(?:(I+|[123])\s*)?([A-Z]+)\b(?:[ .](\d+(?::\d+){0,2}\b))?(?:-(\d+(?::\d+){0,2}\b))?(?:,(\d+(?::\d+){0,2}\b))*(?:;(\d+(?::\d+){0,2}))*(;)*\s*[0-9\-]*\b").unwrap());
//     let new_other_refs: Vec<_> = other_refs
//         .iter()
//         .flat_map(
//             |s| RE.find_iter(s)
//             .map(|m| m.as_str())
//         )
//         // separate each string reference in the vector by % (this is undone below)
//         .map(|s| {
//             let mut s = s.to_string();
//             s.push_str(" %");
//             s
//         })
//         .filter(|r| !r.contains("AND") && !r.contains("and"))
//         .flat_map(|t| {
//             t.split_whitespace()
//             .map(|word| {
//                 if word.chars().any(char::is_alphabetic) {
//                     BibleMap::get_book_code_by_name(word).map(String::from)
//                 } else {
//                     Some(word.to_string())
//                 }
//             }).collect::<Vec<Option<String>>>()
//         })
//         // remove the Option and recollect
//         .filter_map(|opt| opt)
//         .collect::<Vec<String>>()
//         // collect references between %
//         .join(" ")
//         .split('%')
//         .map(|s| s.replace(" ", ""))
//         .map(String::from)
//         .filter(|s| !s.is_empty())
//         .collect();

//     println!("{:?}", new_other_refs);
    
    // // Convert the tokenized refs to integer ids
    // for r in new_other_refs {
    //     // capture the book context (now always the first three characters)
    //     let (book, remainder) = r.split_at(3);

    //     // if verse numbers follow a chapter number, prepend them with the chapter number
    //     let chapter_re = Regex::new(r"(\d+:)(\d+)").unwrap();
    //     // there can be multiple chapter captures in a reference
    //     let mut chap_caps: Vec<String> = Vec::new();
    //     let mut ref_caps: Vec<String> = Vec::new();
    //     for cap in chapter_re.captures_iter(remainder) {
    //         let ref_str = cap[0].to_string();
    //         let chapter_str = cap[1].to_string();
    //         chap_caps.push(chapter_str);
    //         ref_caps.push(ref_str);
    //     }
    //     // println!("{} -- {:?} {:?}", r, chap_caps, ref_caps);

    //     let mut result: Vec<String> = vec![r.to_string()];

    //     for substring in ref_caps {
    //         let mut new_result: Vec<String> = Vec::new();
    //         for part in result {
    //             let ss = substring.as_str();
    //             let parts: Vec<&str> = part.splitn(2, ss).collect();
    //             if parts.len() > 1 {
    //                 new_result.push(parts[0].to_string());
    //                 new_result.push(substring.to_string());
    //                 new_result.push(parts[1].to_string());
    //             } else {
    //                 new_result.push(part.to_string());
    //             }
    //         }
    //         result = new_result;
    //     }

    //     result.retain(|x| x != ""); // Remove empty strings
    //     println!("{:?}, {:?}", result, chap_caps);
    // }
