use std::collections::HashMap;

pub struct BibleMap {

}

impl BibleMap {

    pub fn get_book_id_by_name(name: &str) -> Option<u32> {
        match BibleMap::get_book_code_by_name(name) {
            None => None,
            Some(code) => {
                match BibleMap::get_book_id_by_code(code) {
                    None => None,
                    Some(id) => Some(id)
                }
            }
        }
    }

    pub fn get_book_code_by_name(name: &str) -> Option<&str> {
        // Remove whitespace (including spaces)
        let name = name.replace(&[' ', '\t', '\n', '\r'][..], "");
        let name = regex::Regex::new(r#"[^0-9a-zA-Z]"#).unwrap().replace_all(&name, "");
        let name = name.to_uppercase();

        let mut map = HashMap::new();
        map.insert("GE", "GEN");
        map.insert("GEN", "GEN");
        map.insert("GENESIS", "GEN");
        map.insert("EX", "EXO");
        map.insert("EXO", "EXO");
        map.insert("EXODUS", "EXO");
        map.insert("LEV", "LEV");
        map.insert("LEVITICUS", "LEV");
        map.insert("NU", "NUM");
        map.insert("NUM", "NUM");
        map.insert("NUMBERS", "NUM");
        map.insert("DEU", "DEU");
        map.insert("DT", "DEU");
        map.insert("DEUT", "DEU");
        map.insert("DEUTERONOMY", "DEU");
        map.insert("JOS", "JOS");
        map.insert("JOSHUA", "JOS");
        map.insert("JDG", "JDG");
        map.insert("JUDGES", "JDG");
        map.insert("1SA", "1SA");
        map.insert("1SAM", "1SA");
        map.insert("1SAMUEL", "1SA");
        map.insert("ISAM", "1SA");
        map.insert("ISAMUEL", "1SA");
        map.insert("2SA", "2SA");
        map.insert("2SAM", "2SA");
        map.insert("2SAMUEL", "2SA");
        map.insert("IISA", "2SA");
        map.insert("IISAM", "2SA");
        map.insert("IISAMUEL", "2SA");
        map.insert("RUT", "RUT");
        map.insert("RUTH", "RUT");
        map.insert("1KI", "1KI");
        map.insert("1KINGS", "1KI");
        map.insert("IKI", "1KI");
        map.insert("IKINGS", "1KI");
        map.insert("2KI", "2KI");
        map.insert("2KINGS", "2KI");
        map.insert("IIKI", "2KI");
        map.insert("IIKINGS", "2KI");
        map.insert("1CH", "1CH");
        map.insert("1CHRON", "1CH");
        map.insert("1CHRONICLES", "1CH");
        map.insert("ICH", "1CH");
        map.insert("ICHRON", "1CH");
        map.insert("ICHRONICLES", "1CH");
        map.insert("2CH", "2CH");
        map.insert("2CHRON", "2CH");
        map.insert("2CHRONICLES", "2CH");
        map.insert("IICH", "2CH");
        map.insert("IICHRON", "2CH");
        map.insert("IICHRONICLES", "2CH");
        map.insert("EZR", "EZR");
        map.insert("EZRA", "EZR");
        map.insert("NE", "NEH");
        map.insert("NEH", "NEH");
        map.insert("NEHEMIAH", "NEH");
        map.insert("EST", "EST");
        map.insert("ESTHER", "EST");
        map.insert("JOB", "JOB");
        map.insert("PS", "PSA");
        map.insert("PSA", "PSA");
        map.insert("PSALM", "PSA");
        map.insert("PRO", "PRO");
        map.insert("PR", "PRO");
        map.insert("PROV", "PRO");
        map.insert("PROVERBS", "PRO");
        map.insert("ECC", "ECC");
        map.insert("ECCLES", "ECC");
        map.insert("ECCLESIASTES", "ECC");
        map.insert("SS", "SNG");
        map.insert("SONG", "SNG");
        map.insert("SONGOFSONGS", "SNG");
        map.insert("SONGOFSOLOMON", "SNG");
        map.insert("IS", "ISA");
        map.insert("ISA", "ISA");
        map.insert("ISAIAH", "ISA");
        map.insert("JER", "JER");
        map.insert("JEREMIAH", "JER");
        map.insert("LA", "LAM");
        map.insert("LAM", "LAM");
        map.insert("LAMENTATIONS", "LAM");
        map.insert("EZE", "EZK");
        map.insert("EZK", "EZK");
        map.insert("EZEK", "EZK");
        map.insert("EZEKIEL", "EZK");
        map.insert("DA", "DAN");
        map.insert("DAN", "DAN");
        map.insert("DANIEL", "DAN");
        map.insert("HOS", "HOS");
        map.insert("HOSEA", "HOS");
        map.insert("JOEL", "JOL");
        map.insert("AM", "AMO");
        map.insert("AMO", "AMO");
        map.insert("AMOS", "AMO");
        map.insert("OB", "OBA");
        map.insert("OBA", "OBA");
        map.insert("OBADIAH", "OBA");
        map.insert("JNH", "JON");
        map.insert("JON", "JON");
        map.insert("JONAH", "JON");
        map.insert("MIC", "MIC");
        map.insert("MICAH", "MIC");
        map.insert("NA", "NAM");
        map.insert("NAM", "NAM");
        map.insert("NAHUM", "NAM");
        map.insert("HAB", "HAB");
        map.insert("HAB.", "HAB");
        map.insert("HABAKKUK", "HAB");
        map.insert("ZEP", "ZEP");
        map.insert("ZEPHANIAH", "ZEP");
        map.insert("HAG", "HAG");
        map.insert("HAGGAI", "HAG");
        map.insert("ZEC", "ZEC");
        map.insert("ZECH", "ZEC");
        map.insert("ZECHARIAH", "ZEC");
        map.insert("MAL", "MAL");
        map.insert("MALACHI", "MAL");
        map.insert("MT", "MAT");
        map.insert("MATT", "MAT");
        map.insert("MATTHEW", "MAT");
        map.insert("MK", "MRK");
        map.insert("MARK", "MRK");
        map.insert("LK", "LUK");
        map.insert("LUKE", "LUK");
        map.insert("JN", "JHN");
        map.insert("JOHN", "JHN");
        map.insert("AC", "ACT");
        map.insert("ACTS", "ACT");
        map.insert("RO", "ROM");
        map.insert("ROMANS", "ROM");
        map.insert("1CO", "1CO");
        map.insert("1 COR", "1CO");
        map.insert("1CORINTHIANS", "1CO");
        map.insert("ICO", "1CO");
        map.insert("ICOR", "1CO");
        map.insert("ICORINTHIANS", "1CO");
        map.insert("2CO", "2CO");
        map.insert("2COR", "2CO");
        map.insert("2CORINTHIANS", "2CO");
        map.insert("IICO", "2CO");
        map.insert("IICOR", "2CO");
        map.insert("IICORINTHIANS", "2CO");
        map.insert("GAL", "GAL");
        map.insert("GALATIANS", "GAL");
        map.insert("EPH", "EPH");
        map.insert("EPHESIANS", "EPH");
        map.insert("PHP", "PHP");
        map.insert("PHIL", "PHP");
        map.insert("PHILIPPIANS", "PHP");
        map.insert("COL", "COL");
        map.insert("COLOSSIANS", "COL");
        map.insert("1TH", "1TH");
        map.insert("1THES", "1TH");
        map.insert("1THESSALONIANS", "1TH");
        map.insert("ITH", "1TH");
        map.insert("ITHES", "1TH");
        map.insert("ITHESSALONIANS", "1TH");
        map.insert("2TH", "2TH");
        map.insert("2THES", "2TH");
        map.insert("2THESSALONIANS", "2TH");
        map.insert("IITH", "2TH");
        map.insert("IITHES", "2TH");
        map.insert("IITHESSALONIANS", "2TH");
        map.insert("1TI", "1TI");
        map.insert("1TIM", "1TI");
        map.insert("1TIMOTHY", "1TI");
        map.insert("ITI", "1TI");
        map.insert("ITIM", "1TI");
        map.insert("ITIMOTHY", "1TI");
        map.insert("2TI", "2TI");
        map.insert("2TIM", "2TI");
        map.insert("2TIMOTHY", "2TI");
        map.insert("IITI", "2TI");
        map.insert("IITIM", "2TI");
        map.insert("IITIMOTHY", "2TI");
        map.insert("TIT", "TIT");
        map.insert("TITUS", "TIT");
        map.insert("PHM", "PHM");
        map.insert("PHILEMON", "PHM");
        map.insert("HEB", "HEB");
        map.insert("HEBREWS", "HEB");
        map.insert("JAS", "JAS");
        map.insert("JAMES", "JAS");
        map.insert("1PE", "1PE");
        map.insert("1PET", "1PE");
        map.insert("1PETER", "1PE");
        map.insert("IPE", "1PE");
        map.insert("IPET", "1PE");
        map.insert("IPETER", "1PE");
        map.insert("2PE", "2PE");
        map.insert("2PET", "2PE");
        map.insert("2PETER", "2PE");
        map.insert("IIPE", "2PE");
        map.insert("IIPET", "2PE");
        map.insert("IIPETER", "2PE");
        map.insert("1JN", "1JN");
        map.insert("1JOHN", "1JN");
        map.insert("IJN", "1JN");
        map.insert("IJOHN", "1JN");
        map.insert("2JN", "2JN");
        map.insert("2JOHN", "2JN");
        map.insert("IIJN", "2JN");
        map.insert("IIJOHN", "2JN");
        map.insert("3JN", "3JN");
        map.insert("3JOHN", "3JN");
        map.insert("IIIJN", "3JN");
        map.insert("IIIJOHN", "3JN");
        map.insert("JUDE", "JUD");
        map.insert("REV", "REV");
        map.insert("REVELATION", "REV");

        map.get(&name.as_str()).cloned()
    }

    pub fn get_book_id_by_code(code: &str) -> Option<u32> {
        let map = [
            ("GEN", 1),
            ("EXO", 2),
            ("LEV", 3),
            ("NUM", 4),
            ("DEU", 5),
            ("JOS", 6),
            ("JDG", 7),
            ("RUT", 8),
            ("1SA", 9),
            ("2SA", 10),
            ("1KI", 11),
            ("2KI", 12),
            ("1CH", 13),
            ("2CH", 14),
            ("EZR", 15),
            ("NEH", 16),
            ("EST", 17),
            ("JOB", 18),
            ("PSA", 19),
            ("PRO", 20),
            ("ECC", 21),
            ("SNG", 22),
            ("ISA", 23),
            ("JER", 24),
            ("LAM", 25),
            ("EZK", 26),
            ("DAN", 27),
            ("HOS", 28),
            ("JOL", 29),
            ("AMO", 30),
            ("OBA", 31),
            ("JON", 32),
            ("MIC", 33),
            ("NAM", 34),
            ("HAB", 35),
            ("ZEP", 36),
            ("HAG", 37),
            ("ZEC", 38),
            ("MAL", 39),
            ("MAT", 40),
            ("MRK", 41),
            ("LUK", 42),
            ("JHN", 43),
            ("ACT", 44),
            ("ROM", 45),
            ("1CO", 46),
            ("2CO", 47),
            ("GAL", 48),
            ("EPH", 49),
            ("PHP", 50),
            ("COL", 51),
            ("1TH", 52),
            ("2TH", 53),
            ("1TI", 54),
            ("2TI", 55),
            ("TIT", 56),
            ("PHM", 57),
            ("HEB", 58),
            ("JAS", 59),
            ("1PE", 60),
            ("2PE", 61),
            ("1JN", 62),
            ("2JN", 63),
            ("3JN", 64),
            ("JUD", 65),
            ("REV", 66),
        ];

        let upper_code = code.to_uppercase();
        for (key, value) in map.iter() {
            if key == &upper_code {
                return Some(*value);
            }
        }

        None
    }

}