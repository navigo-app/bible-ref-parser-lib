use crate::parse;
use crate::utils::bible_map::*;

#[test]
fn test_parse() {
	assert_eq!(
		parse("Job, Lev - Num"),
		Some(vec!["18001001-18999999".to_string(), "3001001-4999999".to_string()]));
	assert_eq!(
		parse("Rev. 22.1, Rev 21"),
		Some(vec!["66022001".to_string(), "66021001-66021999".to_string()]));
	assert_eq!(
		parse("Gen 50-Exo 2"),
		Some(vec!["1050001-2002999".to_string()]));
	assert_eq!(
		parse("Jn 3:16-17, Ge 1:1 - Ex 2:1, Lev 9:12"),
		Some(vec!["43003016-43003017".to_string(), "1001001-2002001".to_string(), "3009012".to_string()]));
	assert_eq!(
		parse("2 Peter 1-1 John 2, Romans 5:12, 2 Corinthians 11.12-Galatians 1.5"),
		Some(vec!["61001001-62002999".to_string(), "45005012".to_string(), "47011012-48001005".to_string()]));
	assert_eq!(
		parse("Acts 15-16"),
		Some(vec!["44015001-44016999".to_string()]));
	assert_eq!(
		parse("Genesis 1:1, 2:3"),
		Some(vec!["1001001".to_string(), "1002003".to_string()]));
	assert_eq!(
		parse("Psalm 119:150, Hebrews 3:1-6, Psalm 5"),
		Some(vec!["19119150".to_string(), "58003001-58003006".to_string(), "19005001-19005999".to_string()]));
	assert_eq!(
		parse("Rev. 22.1"),
		Some(vec!["66022001".to_string()]));
	assert_eq!(
		parse("III John 1:3"),
		Some(vec!["64001003".to_string()]));
	assert_eq!(
		parse("Gen. 1"),
		Some(vec!["1001001-1001999".to_string()]));
	assert_eq!(
		parse("Exo. 1;2,3"),
		Some(vec!["2001001-2001999".to_string(), "2002001-2002999".to_string(), "2003001-2003999".to_string()]));
	assert_eq!(
		parse("Dt 1:1,2,4"),
		Some(vec!["5001001".to_string(), "5001002".to_string(), "5001004".to_string()]));
	assert_eq!(
		parse("I Sam 1:1-2:20,23"),
		Some(vec!["9001001-9002020".to_string(), "9002023".to_string()]));
	assert_eq!(
		parse("Song of Solomon 1; Psalms 2:3"),
		Some(vec!["22001001-22001999".to_string(), "19002003".to_string()]));
	assert_eq!(
		parse("I Cor. 1 & Ps. 1"),
		Some(vec!["46001001-46001999".to_string(), "19001001-19001999".to_string()]));
	assert_eq!(
		parse("Judges 1 and Prov. 2"),
		Some(vec!["7001001-7001999".to_string(), "20002001-20002999".to_string()]));
	assert_eq!(
		parse("Gen 7:21,19:17,20-27"),
		Some(vec!["1007021".to_string(), "1019017".to_string(), "1019020-1019027".to_string()]));
	assert_eq!(
		parse("Mark 1,2,3:27"),
		Some(vec!["41001001-41001999".to_string(), "41002001-41002999".to_string(), "41003027".to_string()]));
	assert_eq!(
		parse("Gen 1:1-20; 24"),
		Some(vec!["1001001-1001020".to_string(), "1024001-1024999".to_string()]));
	assert_eq!(
		parse("2 Jn 1"),
		Some(vec!["63001001-63001999".to_string()]));
	assert_eq!(
		parse("Psalm 6:22, 1 John 3:15"),
		Some(vec!["19006022".to_string(), "62003015".to_string()]));
	assert_eq!(
		parse("GENESIS 7:21,19:17,20-27;1 JOHN 1,2,3:27"),
		Some(vec!["1007021".to_string(), "1019017".to_string(), "1019020-1019027".to_string(), "62001001-62001999".to_string(), "62002001-62002999".to_string(), "62003027".to_string()]));
	assert_eq!(
		parse("Acts 2:3,16-17,33, 10:44-45; Matt 3:16b-17a; Luke 3:22a"),
		Some(vec!["44002003".to_string(), "44002016-44002017".to_string(), "44002033".to_string(), "44010044-44010045".to_string(), "40003016-40003017".to_string(), "42003022".to_string()]));
	assert_eq!(
		parse("I Sa 1:1-2:20"),
		Some(vec!["23001001-23002020".to_string()]));
	assert_eq!(
		parse("Ezra 1:3-2"),
		Some(vec!["15001003".to_string()]));
	assert_eq!(
		parse("Ruth 3-2"),
		Some(vec!["8003001-8003999".to_string()]));
	assert_eq!(
		parse("Numbers 1 - Leviticus 30"),
		Some(vec!["4001001-4001999".to_string()]));
	assert_eq!(
		parse("John 3:16, Deut 30 - Josiah 1"),
		Some(vec!["43003016".to_string()]));
	assert_eq!(
		parse("matt3 13 17"),
		None);
	assert_eq!(
		parse("Joshua 2415-16"),
		None);
	assert_eq!(
		parse("2 Sam 1:1220"),
		None);
	assert_eq!(
		parse("Josiah 40:1 - Judges 3:7"),
		None);
	assert_eq!(
		parse("Josiah 40 - Judges 3"),
		None);
	assert_eq!(
		parse("Deut 30:4 - Josiah 1:2"),
		None);
	assert_eq!(
		parse("John 3:16:17"),
		None);
	assert_eq!(
		parse("Garbage 3:16"),
		None);
	assert_eq!(
		parse("John E:16"),
		None);
	assert_eq!(
		parse("1,728"),
		None);
}

#[test]
fn test_utils() {
	let code = BibleMap::get_book_code_by_id(66001001);
	assert_eq!(
		code,
		Some("REV".to_string())
	);

	let code = BibleMap::get_book_code_by_id(10);
	assert_eq!(
		code,
		Some("2SA".to_string())
	);

	let code = BibleMap::get_book_code_by_id(11);
	assert_eq!(
		code,
		Some("1KI".to_string())
	);

	let code = BibleMap::get_book_code_by_id(1);
	assert_eq!(
		code,
		Some("GEN".to_string())
	);

	let code = BibleMap::get_book_code_by_id(01);
	assert_eq!(
		code,
		Some("GEN".to_string())
	);

	let code = BibleMap::get_book_code_by_id(8001001);
	assert_eq!(
		code,
		Some("RUT".to_string())
	);


	let code = BibleMap::get_book_code_by_id(65001005);
	assert_eq!(
		code,
		Some("JUD".to_string())
	);

}