use std::collections::HashMap;

pub struct BibleMap {}

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
        map.insert("GN", "GEN");
        map.insert("GEN", "GEN");
        map.insert("GENESIS", "GEN");
        map.insert("EX", "EXO");
        map.insert("EXO", "EXO");
        map.insert("EXOD", "EXO");
        map.insert("EXODUS", "EXO");
        map.insert("LV", "LEV");
        map.insert("LEV", "LEV");
        map.insert("LEVITICUS", "LEV");
        map.insert("NU", "NUM");
        map.insert("NM", "NUM");
        map.insert("NUM", "NUM");
        map.insert("NUMBERS", "NUM");
        map.insert("DT", "DEU");
        map.insert("DEU", "DEU");
        map.insert("DEUT", "DEU");
        map.insert("DEUTERONOMY", "DEU");
        map.insert("JOS", "JOS");
        map.insert("JOSH", "JOS");
        map.insert("JOSHUA", "JOS");
        map.insert("JDG", "JDG");
        map.insert("JUDG", "JDG");
        map.insert("JUDGES", "JDG");
        map.insert("RU", "RUT");
        map.insert("RUT", "RUT");
        map.insert("RUTH", "RUT");
        map.insert("1SA", "1SA");
        map.insert("1SM", "1SA");
        map.insert("1SAM", "1SA");
        map.insert("1SAMUEL", "1SA");
        map.insert("ISAM", "1SA");
        map.insert("ISAMUEL", "1SA");
        map.insert("2SA", "2SA");
        map.insert("2SM", "2SA");
        map.insert("2SAM", "2SA");
        map.insert("2SAMUEL", "2SA");
        map.insert("IISA", "2SA");
        map.insert("IISAM", "2SA");
        map.insert("IISAMUEL", "2SA");
        map.insert("1KG", "1KI");
        map.insert("1KI", "1KI");
        map.insert("1KGS", "1KI");
        map.insert("1KINGS", "1KI");
        map.insert("IKI", "1KI");
        map.insert("IKINGS", "1KI");
        map.insert("2KG", "2KI");
        map.insert("2KI", "2KI");
        map.insert("2KGS", "2KI");
        map.insert("2KINGS", "2KI");
        map.insert("IIKI", "2KI");
        map.insert("IIKINGS", "2KI");
        map.insert("1CH", "1CH");
        map.insert("1CHR", "1CH");
        map.insert("1CHRON", "1CH");
        map.insert("1CHRONICLES", "1CH");
        map.insert("ICH", "1CH");
        map.insert("ICHRON", "1CH");
        map.insert("ICHRONICLES", "1CH");
        map.insert("2CH", "2CH");
        map.insert("2CHR", "2CH");
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
        map.insert("ESTH", "EST");
        map.insert("ESTHER", "EST");
        map.insert("JB", "JOB");
        map.insert("JOB", "JOB");
        map.insert("PS", "PSA");
        map.insert("PSA", "PSA");
        map.insert("PSALM", "PSA");
        map.insert("PSALMS", "PSA");
        map.insert("PR", "PRO");
        map.insert("PRO", "PRO");
        map.insert("PRV", "PRO");
        map.insert("PROV", "PRO");
        map.insert("PROVERBS", "PRO");
        map.insert("ECC", "ECC");
        map.insert("ECCL", "ECC");
        map.insert("ECCLES", "ECC");
        map.insert("ECCLESIASTES", "ECC");
        map.insert("SS", "SNG");
        map.insert("SNG", "SNG");
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
        map.insert("EZ", "EZK");
        map.insert("EZE", "EZK");
        map.insert("EZK", "EZK");
        map.insert("EZEK", "EZK");
        map.insert("EZEKIEL", "EZK");
        map.insert("DA", "DAN");
        map.insert("DN", "DAN");
        map.insert("DAN", "DAN");
        map.insert("DANIEL", "DAN");
        map.insert("HOS", "HOS");
        map.insert("HOSEA", "HOS");
        map.insert("JL", "JOL");
        map.insert("JOE", "JOL");
        map.insert("JOEL", "JOL");
        map.insert("AM", "AMO");
        map.insert("AMO", "AMO");
        map.insert("AMOS", "AMO");
        map.insert("OB", "OBA");
        map.insert("OBA", "OBA");
        map.insert("OBAD", "OBA");
        map.insert("OBADIAH", "OBA");
        map.insert("JNH", "JON");
        map.insert("JON", "JON");
        map.insert("JONAH", "JON");
        map.insert("MI", "MIC");
        map.insert("MIC", "MIC");
        map.insert("MICAH", "MIC");
        map.insert("NA", "NAM");
        map.insert("NAH", "NAM");
        map.insert("NAM", "NAM");
        map.insert("NAHUM", "NAM");
        map.insert("HAB", "HAB");
        map.insert("HABAKKUK", "HAB");
        map.insert("ZEP", "ZEP");
        map.insert("ZEPH", "ZEP");
        map.insert("ZEPHANIAH", "ZEP");
        map.insert("HG", "HAG");
        map.insert("HAG", "HAG");
        map.insert("HAGGAI", "HAG");
        map.insert("ZEC", "ZEC");
        map.insert("ZECH", "ZEC");
        map.insert("ZECHARIAH", "ZEC");
        map.insert("MAL", "MAL");
        map.insert("MALACHI", "MAL");
        map.insert("MT", "MAT");
        map.insert("MAT", "MAT");
        map.insert("MATT", "MAT");
        map.insert("MATTHEW", "MAT");
        map.insert("MK", "MRK");
        map.insert("MAR", "MRK");
        map.insert("MARK", "MRK");
        map.insert("LK", "LUK");
        map.insert("LUK", "LUK");
        map.insert("LUKE", "LUK");
        map.insert("JN", "JHN");
        map.insert("JOHN", "JHN");
        map.insert("AC", "ACT");
        map.insert("ACT", "ACT");
        map.insert("ACTS", "ACT");
        map.insert("RO", "ROM");
        map.insert("ROM", "ROM");
        map.insert("ROMANS", "ROM");
        map.insert("1CO", "1CO");
        map.insert("1COR", "1CO");
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
        map.insert("1THESS", "1TH");
        map.insert("1THESSALONIANS", "1TH");
        map.insert("ITH", "1TH");
        map.insert("ITHES", "1TH");
        map.insert("ITHESSALONIANS", "1TH");
        map.insert("2TH", "2TH");
        map.insert("2THES", "2TH");
        map.insert("2THESS", "2TH");
        map.insert("2THESSALONIANS", "2TH");
        map.insert("IITH", "2TH");
        map.insert("IITHES", "2TH");
        map.insert("IITHESSALONIANS", "2TH");
        map.insert("1TI", "1TI");
        map.insert("1TM", "1TI");
        map.insert("1TIM", "1TI");
        map.insert("1TIMOTHY", "1TI");
        map.insert("ITI", "1TI");
        map.insert("ITIM", "1TI");
        map.insert("ITIMOTHY", "1TI");
        map.insert("2TI", "2TI");
        map.insert("2TM", "2TI");
        map.insert("2TIM", "2TI");
        map.insert("2TIMOTHY", "2TI");
        map.insert("IITI", "2TI");
        map.insert("IITIM", "2TI");
        map.insert("IITIMOTHY", "2TI");
        map.insert("TI", "TIT");
        map.insert("TIT", "TIT");
        map.insert("TITUS", "TIT");
        map.insert("PHM", "PHM");
        map.insert("PHLM", "PHM");
        map.insert("PHILEM", "PHM");
        map.insert("PHILEMON", "PHM");
        map.insert("HEB", "HEB");
        map.insert("HEBREWS", "HEB");
        map.insert("JAS", "JAS");
        map.insert("JAMES", "JAS");
        map.insert("1PE", "1PE");
        map.insert("1PT", "1PE");
        map.insert("1PET", "1PE");
        map.insert("1PETER", "1PE");
        map.insert("IPE", "1PE");
        map.insert("IPET", "1PE");
        map.insert("IPETER", "1PE");
        map.insert("2PE", "2PE");
        map.insert("2PT", "2PE");
        map.insert("2PET", "2PE");
        map.insert("2PETER", "2PE");
        map.insert("IIPE", "2PE");
        map.insert("IIPET", "2PE");
        map.insert("IIPETER", "2PE");
        map.insert("1JN", "1JN");
        map.insert("1JOHN", "1JN");
        map.insert("IJN", "1JN");
        map.insert("1JO", "1JN");
        map.insert("IJOHN", "1JN");
        map.insert("2JN", "2JN");
        map.insert("2JO", "2JN");
        map.insert("2JOHN", "2JN");
        map.insert("IIJN", "2JN");
        map.insert("IIJOHN", "2JN");
        map.insert("3JN", "3JN");
        map.insert("3JO", "3JN");
        map.insert("3JOHN", "3JN");
        map.insert("IIIJN", "3JN");
        map.insert("IIIJOHN", "3JN");
        map.insert("JUD", "JUD");
        map.insert("JUDE", "JUD");
        map.insert("RV", "REV");
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

    pub fn get_book_code_by_id(mut ref_id: u32) -> Option<String> {

        //Here are the possible valid integers a book could be gotten from.

        // 8        << Ruth             (1 digit)
        // 8001     << Ruth 1           (4 digits)
        // 8001001  << Ruth 1:1         (7 Digits)
        // 66       << Revelation       (2 digits)
        // 66001    << Revelation 1     (5 Digits)
        // 66001001 << Revelation 1:1   (8 Digits)

        // This reduces any ref_id down to the book code
        // If the ref_id is a 2, 5, or 8-digit number then the first two digits are the book code.
        // Otherwise (if the ref_id is a 1, 4 or 7-digit number) the first digit is the book code.
        if ref_id >= 10000000 || (ref_id >= 10000 && ref_id <= 99999) || (ref_id >= 10 && ref_id <= 99) {
            while ref_id >= 100 { ref_id /= 10; }
        } else {
            while ref_id >= 10 { ref_id /= 10; }
        }

        let map = [
            (1, "GEN"),
            (2, "EXO"),
            (3, "LEV"),
            (4, "NUM"),
            (5, "DEU"),
            (6, "JOS"),
            (7, "JDG"),
            (8, "RUT"),
            (9, "1SA"),
            (10, "2SA"),
            (11, "1KI"),
            (12, "2KI"),
            (13, "1CH"),
            (14, "2CH"),
            (15, "EZR"),
            (16, "NEH"),
            (17, "EST"),
            (18, "JOB"),
            (19, "PSA"),
            (20, "PRO"),
            (21, "ECC"),
            (22, "SNG"),
            (23, "ISA"),
            (24, "JER"),
            (25, "LAM"),
            (26, "EZK"),
            (27, "DAN"),
            (28, "HOS"),
            (29, "JOL"),
            (30, "AMO"),
            (31, "OBA"),
            (32, "JON"),
            (33, "MIC"),
            (34, "NAM"),
            (35, "HAB"),
            (36, "ZEP"),
            (37, "HAG"),
            (38, "ZEC"),
            (39, "MAL"),
            (40, "MAT"),
            (41, "MRK"),
            (42, "LUK"),
            (43, "JHN"),
            (44, "ACT"),
            (45, "ROM"),
            (46, "1CO"),
            (47, "2CO"),
            (48, "GAL"),
            (49, "EPH"),
            (50, "PHP"),
            (51, "COL"),
            (52, "1TH"),
            (53, "2TH"),
            (54, "1TI"),
            (55, "2TI"),
            (56, "TIT"),
            (57, "PHM"),
            (58, "HEB"),
            (59, "JAS"),
            (60, "1PE"),
            (61, "2PE"),
            (62, "1JN"),
            (63, "2JN"),
            (64, "3JN"),
            (65, "JUD"),
            (66, "REV"),
        ];

        for (key, value) in map.iter() {
            if key == &ref_id {
                return Some(value.to_string());
            }
        }

        None
    }

    pub fn get_book_name_by_id(book_id: u32) -> Option<String> {
        let map = [
            (1, "Genesis"),
            (2, "Exodus"),
            (3, "Leviticus"),
            (4, "Numbers"),
            (5, "Deuteronomy"),
            (6, "Joshua"),
            (7, "Judges"),
            (8, "Ruth"),
            (9, "1 Samuel"),
            (10, "2 Samuel"),
            (11, "1 Kings"),
            (12, "2 Kings"),
            (13, "1 Chronicles"),
            (14, "2 Chronicles"),
            (15, "Ezra"),
            (16, "Nehemiah"),
            (17, "Esther"),
            (18, "Job"),
            (19, "Psalms"),
            (20, "Proverbs"),
            (21, "Ecclesiastes"),
            (22, "Song of Solomon"),
            (23, "Isaiah"),
            (24, "Jeremiah"),
            (25, "Lamentations"),
            (26, "Ezekiel"),
            (27, "Daniel"),
            (28, "Hosea"),
            (29, "Joel"),
            (30, "Amos"),
            (31, "Obadiah"),
            (32, "Jonah"),
            (33, "Micah"),
            (34, "Nahum"),
            (35, "Habakkuk"),
            (36, "Zephaniah"),
            (37, "Haggai"),
            (38, "Zechariah"),
            (39, "Malachi"),
            (40, "Matthew"),
            (41, "Mark"),
            (42, "Luke"),
            (43, "John"),
            (44, "Acts"),
            (45, "Romans"),
            (46, "1 Corinthians"),
            (47, "2 Corinthians"),
            (48, "Galatians"),
            (49, "Ephesians"),
            (50, "Philippians"),
            (51, "Colossians"),
            (52, "1 Thessalonians"),
            (53, "2 Thessalonians"),
            (54, "1 Timothy"),
            (55, "2 Timothy"),
            (56, "Titus"),
            (57, "Philemon"),
            (58, "Hebrews"),
            (59, "James"),
            (60, "1 Peter"),
            (61, "2 Peter"),
            (62, "1 John"),
            (63, "2 John"),
            (64, "3 John"),
            (65, "Jude"),
            (66, "Revelation"),
        ];

        for (key, value) in map.iter() {
            if key == &book_id {
                return Some(value.to_string());
            }
        }

        None
    }

    pub fn get_chinese_book_name_by_id(book_id: u32) -> Option<String> {
        let map = [
            (1, "创世记"),
            (2, "出埃及记"),
            (3, "利未记"),
            (4, "民数记"),
            (5, "申命记"),
            (6, "约书亚记"),
            (7, "士师记"),
            (8, "路得记"),
            (9, "撒母耳记上"),
            (10, "撒母耳记下"),
            (11, "列王纪上"),
            (12, "列王纪下"),
            (13, "历代志上"),
            (14, "历代志下"),
            (15, "以斯拉记"),
            (16, "尼希米记"),
            (17, "以斯帖记"),
            (18, "约伯记"),
            (19, "诗篇"),
            (20, "箴言"),
            (21, "传道书"),
            (22, "雅歌"),
            (23, "以赛亚书"),
            (24, "耶利米书"),
            (25, "耶利米哀歌"),
            (26, "以西结书"),
            (27, "但以理书"),
            (28, "何西阿书"),
            (29, "约珥书"),
            (30, "阿摩司书"),
            (31, "俄巴底亚书"),
            (32, "约拿书"),
            (33, "弥迦书"),
            (34, "那鸿书"),
            (35, "哈巴谷书"),
            (36, "西番雅书"),
            (37, "哈该书"),
            (38, "撒迦利亚书"),
            (39, "玛拉基书"),
            (40, "马太福音"),
            (41, "马可福音"),
            (42, "路加福音"),
            (43, "约翰福音"),
            (44, "使徒行传"),
            (45, "罗马书"),
            (46, "哥林多前书"),
            (47, "哥林多后书"),
            (48, "加拉太书"),
            (49, "以弗所书"),
            (50, "腓立比书"),
            (51, "歌罗西书"),
            (52, "帖撒罗尼迦前书"),
            (53, "帖撒罗尼迦后书"),
            (54, "提摩太前书"),
            (55, "提摩太后书"),
            (56, "提多书"),
            (57, "腓利门书"),
            (58, "希伯来书"),
            (59, "雅各书"),
            (60, "彼得前书"),
            (61, "彼得后书"),
            (62, "约翰一书"),
            (63, "约翰二书"),
            (64, "约翰三书"),
            (65, "犹大书"),
            (66, "启示录"),
        ];

        for (key, value) in map.iter() {
            if key == &book_id {
                return Some(value.to_string());
            }
        }

        None
    }

    pub fn get_chapter_verse_book_by_id(ref_id: &u32) -> Option<(u32, u32, u32)> {
        let ref_id = &ref_id.to_string();
        let ref_id_length = ref_id.len();
        if ref_id_length < 7 {
            return None;
        }

        let (book_chap_code, verse_code) = ref_id.split_at(ref_id_length - 3);
        let verse = verse_code.parse::<u32>().unwrap_or(0);

        let (book_code, chapter_code) = book_chap_code.split_at(book_chap_code.len() - 3);

        let chapter = chapter_code.parse::<u32>().unwrap_or(0);
        let book_id = book_code.parse::<u32>().unwrap_or(0);

        return Some((chapter, verse, book_id))
    }

    pub fn get_ref_by_id(ref_id: u32) -> Option<String> {
        let (chapter, verse, book_id) =
            Self::get_chapter_verse_book_by_id(&ref_id).unwrap();

        let book_code = Self::get_book_code_by_id(book_id);

        if book_code == None {
            return None
        } else {
            return Some(format!("{} {}:{}", book_code.unwrap_or_default(), chapter, verse))
        }
    }

    pub fn get_human_ref_by_id(ref_id: u32) -> Option<String> {
        let (chapter, verse, book_id) =
            Self::get_chapter_verse_book_by_id(&ref_id).unwrap();
        let book_code = Self::get_book_name_by_id(book_id);

        if book_code == None {
            return None
        } else {
            return Some(format!("{} {}:{}", book_code.unwrap_or_default(), chapter, verse))
        }
    }

    pub fn get_chinese_ref_by_id(ref_id: u32) -> Option<String> {
        let (chapter, verse, book_id) =
            Self::get_chapter_verse_book_by_id(&ref_id).unwrap();
        let book_code = Self::get_chinese_book_name_by_id(book_id);

        if book_code == None {
            return None
        } else {
            return Some(format!("{} {}:{}", book_code.unwrap_or_default(), chapter, verse))
        }
    }
}
