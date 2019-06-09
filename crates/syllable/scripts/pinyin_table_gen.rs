

pub(crate) const _________: &'static str = "";

// ∅ b p m f d t n l g k h j q x zh ch sh r z c s
#[rustfmt::skip]
pub static PINYIN_TABLE: [&'static str; 1012] = [
//        ∅,         b,         p,         m,         f,         d,         t,         n,         l,         g,         k,         h,         j,         q,         x,        zh,        ch,        sh,         r,         z,         c,         s,
// Group a Finals
  _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,     "zhi",     "chi",     "shi",      "ri",      "zi",      "ci",      "si",
        "a",      "ba",      "pa",      "ma",      "fa",      "da",      "ta",      "na",      "la",      "ga",      "ka",      "ha", _________, _________, _________,     "zha",     "cha",     "sha", _________,      "za",      "ca",      "sa",
        "e", _________, _________,      "me", _________,      "de",      "te",      "ne",      "le",      "ge",      "ke",      "he", _________, _________, _________,     "zhe",     "che",     "she",      "re",      "ze",      "ce",      "se",
        "ê", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
       "er", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
       "ai",     "bai",     "pai",     "mai", _________,     "dai",     "tai",     "nai",     "lai",     "gai",     "kai",     "hai", _________, _________, _________,    "zhai",    "chai",    "shai", _________,     "zai",     "cai",     "sai",
       "ei",     "bei",     "pei",     "mei",     "fei",     "dei",     "tei",     "nei",     "lei",     "gei",     "kei",     "hei", _________, _________, _________,    "zhei", _________,    "shei", _________,     "zei", _________,     "sei",
       "ao",     "bao",     "pao",     "mao", _________,     "dao",     "tao",     "nao",     "lao",     "gao",     "kao",     "hao", _________, _________, _________,    "zhao",    "chao",    "shao",     "rao",     "zao",     "cao",     "sao",
       "ou", _________,     "pou",     "mou",     "fou",     "dou",     "tou",     "nou",     "lou",     "gou",     "kou",     "hou", _________, _________, _________,    "zhou",    "chou",    "shou",     "rou",     "zou",     "cou",     "sou",
       "an",     "ban",     "pan",     "man",     "fan",     "dan",     "tan",     "nan",     "lan",     "gan",     "kan",     "han", _________, _________, _________,    "zhan",    "chan",    "shan",     "ran",     "zan",     "can",     "san",
       "en",     "ben",     "pen",     "men",     "fen",     "den", _________,     "nen", _________,     "gen",     "ken",     "hen", _________, _________, _________,    "zhen",    "chen",    "shen",     "ren",     "zen",     "cen",     "sen",
      "ang",    "bang",    "pang",    "mang",    "fang",    "dang",    "tang",    "nang",    "lang",    "gang",    "kang",    "hang", _________, _________, _________,   "zhang",   "chang",   "shang",    "rang",    "zang",    "cang",    "sang",
      "eng",    "beng",    "peng",    "meng",    "feng",    "deng",    "teng",    "neng",    "leng",    "geng",    "keng",    "heng", _________, _________, _________,   "zheng",   "cheng",   "sheng",    "reng",    "zeng",    "ceng",    "seng",
  _________, _________, _________, _________, _________,    "dong",    "tong",    "nong",    "long",    "gong",    "kong",    "hong", _________, _________, _________,   "zhong",   "chong",   "shong",    "rong",    "zong",    "cong",    "song",

// 特殊音节组合
        "n", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,      "hn", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
        "m", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,      "hm", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
       "ng", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,     "hng", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,

// Group i Finals 
       "yi",      "bi",      "pi",      "mi", _________,      "di",      "ti",      "ni",      "li", _________, _________, _________,      "ji",      "qi",      "xi", _________, _________, _________, _________, _________, _________, _________,
       "ya", _________, _________, _________, _________,     "dia", _________,     "nia",     "lia", _________, _________, _________,     "jia",     "qia",     "xia", _________, _________, _________, _________, _________, _________, _________,
       "yo", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
       "ye",     "bie",     "pie",     "mie", _________,     "die",     "tie",     "nie",     "lie", _________, _________, _________,     "jie",     "qie",     "xie", _________, _________, _________, _________, _________, _________, _________,
      "yao",    "biao",    "piao",    "miao",    "fiao",    "diao",    "tiao",    "niao",    "liao", _________, _________, _________,    "jiao",    "qiao",    "xiao", _________, _________, _________, _________, _________, _________, _________,
      "you", _________, _________,     "miu", _________,     "diu", _________,     "niu",     "liu", _________, _________, _________,     "jiu",     "qiu",     "xiu", _________, _________, _________, _________, _________, _________, _________,
      "yan",    "bian",    "pian",    "mian", _________,    "dian",    "tian",    "nian",    "lian", _________, _________, _________,    "jian",    "qian",    "xian", _________, _________, _________, _________, _________, _________, _________,
      "yin",     "bin",     "pin",     "min", _________, _________, _________,     "nin",     "lin", _________, _________, _________,     "jin",     "qin",     "xin", _________, _________, _________, _________, _________, _________, _________,
     "yang",   "biang", _________, _________, _________,   "diang", _________,   "niang",   "liang", _________, _________, _________,   "jiang",   "qiang",   "xiang", _________, _________, _________, _________, _________, _________, _________,
     "ying",    "bing",    "ping",    "ming", _________,    "ding",    "ting",    "ning",    "ling", _________, _________, _________,    "jing",    "qing",    "xing", _________, _________, _________, _________, _________, _________, _________,
     "yong", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,   "jiong",   "qiong",   "xiong", _________, _________, _________, _________, _________, _________, _________,

// Group u Finals
       "wu",      "bu",      "pu",      "mu",      "fu",      "du",      "tu",      "nu",      "lu",      "gu",      "ku",      "hu", _________, _________, _________,     "zhu",     "chu",     "shu",      "ru",      "zu",      "cu",      "su",
       "wa", _________, _________, _________, _________, _________, _________, _________, _________,     "gua",     "kua",     "hua", _________, _________, _________,    "zhua",    "chua",    "shua",     "rua", _________, _________, _________,
       "wo",      "bo",      "po",      "mo",      "fo",     "duo",     "tuo",     "nuo",     "luo",     "guo",     "kuo",     "huo", _________, _________, _________,    "zhuo",    "chuo",    "shuo",     "ruo",     "zuo",     "cuo",     "suo",
      "wai", _________, _________, _________, _________, _________, _________, _________, _________,    "guai",    "kuai",    "huai", _________, _________, _________,   "zhuai",   "chuai",   "shuai", _________, _________, _________, _________,
      "wei", _________, _________, _________, _________,     "dui",     "tui", _________, _________,     "gui",     "kui",     "hui", _________, _________, _________,    "zhui",    "chui",    "shui",     "rui",     "zui",     "cui",     "sui",
      "wan", _________, _________, _________, _________,    "duan",    "tuan",    "nuan",    "luan",    "guan",    "kuan",    "huan", _________, _________, _________,   "zhuan",   "chuan",   "shuan",    "ruan",    "zuan",    "cuan",    "suan",
      "wen", _________, _________, _________, _________,     "dun",     "tun",     "nun",     "lun",     "gun",     "kun",     "hun", _________, _________, _________,    "zhun",    "chun",    "shun",     "run",     "zun",     "cun",     "sun",
     "wang", _________, _________, _________, _________, _________, _________, _________, _________,   "guang",   "kuang",   "huang", _________, _________, _________,  "zhuang",  "chuang",  "shuang", _________, _________, _________, _________,
     "weng", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,

// Group ü Finals
       "yu", _________, _________, _________, _________, _________, _________,      "nü",      "lü", _________, _________, _________,      "ju",      "qu",      "xu", _________, _________, _________, _________, _________, _________, _________,
      "yue", _________, _________, _________, _________, _________, _________,     "nüe",     "lüe", _________, _________, _________,     "jue",     "que",     "xue", _________, _________, _________, _________, _________, _________, _________,
     "yuan", _________, _________, _________, _________, _________, _________, _________,    "lüan", _________, _________, _________,    "juan",    "quan",    "xuan", _________, _________, _________, _________, _________, _________, _________,
      "yun", _________, _________, _________, _________, _________, _________, _________,     "lün", _________, _________, _________,     "jun",     "qun",     "xun", _________, _________, _________, _________, _________, _________, _________,

// 儿化音节 (er)    
      "wor",    "banr",     "pir",   "mianr",     "fur",   "dianr",   "tangr",     "nar", _________,     "ger",    "kour",    "hair",    "jinr", _________,    "xiar",    "zher", _________,    "shir", _________, _________, _________, _________,
     "wanr", _________, _________, _________, _________,   "dingr",    "tuir",     "nür", _________,    "ganr",   "kongr",    "haor", _________, _________,   "xianr", _________, _________,   "shuir", _________, _________, _________, _________,
  _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,    "huar", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
  _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,    "huor", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,
  _________, _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,    "huir", _________, _________, _________, _________, _________, _________, _________, _________, _________, _________,

];

// 韵母表
#[rustfmt::skip]
pub static RHYME_TABLE: [&'static str; 40] = [
    // Group A
    "a", "e", "ê", "er", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong",

    // 规范未提及的韵母，属于早期发音
    "n", "m", "ng",

    // Group I
    "i", "ia", "io", "ie", "iao", "iou", "ian", "in", "iang", "ing", "iong",
// 无声母时改写为:
//  yi    ya    yo    ye    yao    you    yan   yin    yang   ying    yong

    // Group U
    "u", "ua", "uo", "uai", "uei", "uan", "uen", "uang", "ueng", 
// 无声母时改写为:
//  wu    wa    wo    wai    wei    wan    wen    wang    weng

    // Group Ü
    "ü", "üe", "üan", "ün",
// 无声母时改写为:
//  yu   yue   yuan   yun     // ü 上两点省略；但是跟声母 l 、 n 拼的时候，仍然写成 lü （吕）、 nü （女）。
// 和声母 `J/Q/X` 组合时 字母 ü 上面两点省略
//  ju   jue   juan   jun
//  qu   que   quan   qun
//  xu   xue   xuan   xun
// 和声母 `L/N` 组合时
//  lü   lüe   lüān   lǖn
//  nü   nüē
];

#[rustfmt::skip]
pub static SIMPLIFIED_RHYME_TABLE: [&'static str; 9] = [
    "ang", "eng", "ong",   // 10 ..= 12
    "ng",                  // 15
    "iang", "ing", "iong", // 24 ..= 26
    "uang", "ueng",        // 34 ..= 35
];

// 汉语拼音中标声调位置的规则如下：
// 
// 1. 如果有 a，则标在 a 上。
// 2. 如果没有 a，但有 o 或 e，则标在这两个字母上。这两个字母不会同时出现。
// 3. 如果也没有 o 和 e，则一定有 i、u 或 ü。如果 i 和 u 同时出现，则标在第二个韵母上。
//    这是特别针对 ui 和 iu 而言的（这两个音的实际读音应该是 uei 和 iou ）。
//    如果 i 和 u 不同时出现，则标在出现的那个韵母上。
// 
// "a", "ā", "á", "ǎ", "à",
// "e", "ē", "é", "ě", "è",
// "i", "ī", "í", "ǐ", "ì",
// "m", "m̄", "ḿ", "m̌", "m̀",
// "n", "n̄", "ń", "ň", "ǹ",
// "o", "ō", "ó", "ǒ", "ò",
// "u", "ū", "ú", "ǔ", "ù",
// "ê", "ê̄", "ế", "ê̌", "ề",
// "ü", "ǖ", "ǘ", "ǚ", "ǜ",

fn search_vowel(pinyin: &'static str) -> (&'static str, usize, usize) {
    let a_index = pinyin.find("a");
    let o_index = pinyin.find("o");
    let e_index = pinyin.find("e");

    let i_index = pinyin.find("i");
    let u_index = pinyin.find("u");
    let u2_index = pinyin.find("ü");

    let (index, size) = if a_index.is_some() {
        (a_index.unwrap(), 1)
    } else if o_index.is_some() {
        (o_index.unwrap(), 1)
    } else if e_index.is_some() {
        (e_index.unwrap(), 1)
    } else {
        if i_index.is_some() && u_index.is_some() {
            assert!(u2_index.is_none());
            (std::cmp::max(i_index.unwrap(), u_index.unwrap()), 1)
        } else {
            if i_index.is_some() {
                assert!(u_index.is_none());
                assert!(u2_index.is_none());

                (i_index.unwrap(), 1)
            } else if u_index.is_some() {
                assert!(i_index.is_none());
                assert!(u2_index.is_none());
                
                (u_index.unwrap(), 1)
            } else if u2_index.is_some() {
                assert!(i_index.is_none());
                assert!(u_index.is_none());

                (u2_index.unwrap(), 2)
            } else {
                match pinyin {
                    "ê"   => (0, 2),
                    "n"   => (0, 1),
                    "m"   => (0, 1),
                    "ng"  => (0, 1),
                    "hn"  => (1, 1),
                    "hm"  => (1, 1),
                    "hng" => (1, 1),
                    _     => panic!("[ERROR] 无法确定音节 `{}` 的元音！", pinyin),
                }
            }
        }
    };

    let start = index;
    let end = index + size;
    let vowel = &pinyin[start..end];

    (vowel, start, end)
}

pub enum Style {
    Symbol,
    Number1,
    Number2,
}

fn pinyin_table_codegen(tone_index: usize, style: Style) {
    let mut pos = 0usize;
    for pinyin in PINYIN_TABLE.iter() {
        if pos == 0 {
            print!("  ");
        }

        if pos != 0 && pos % 22 == 0 {
            print!("\n  ");
        }

        if pinyin == &_________ {
            print!("_________, ");
        } else {
            let (vowel, _start, _end) = search_vowel(&pinyin);

            let pinyin_with_tone = match style {
                Style::Symbol => {
                    let vowels = [
                        [ "a", "ā", "á", "ǎ", "à", ],
                        [ "e", "ē", "é", "ě", "è", ],
                        [ "i", "ī", "í", "ǐ", "ì", ],
                        [ "m", "m̄", "ḿ", "m̌", "m̀", ],
                        [ "n", "n̄", "ń", "ň", "ǹ", ],
                        [ "o", "ō", "ó", "ǒ", "ò", ],
                        [ "u", "ū", "ú", "ǔ", "ù", ],
                        [ "ê", "ê̄", "ế", "ê̌", "ề", ],
                        [ "ü", "ǖ", "ǘ", "ǚ", "ǜ", ],
                    ];

                    match vowel {
                        "a" => pinyin.replace(vowel, vowels[0][tone_index]),
                        "e" => pinyin.replace(vowel, vowels[1][tone_index]),
                        "i" => pinyin.replace(vowel, vowels[2][tone_index]),
                        "m" => pinyin.replace(vowel, vowels[3][tone_index]),
                        "n" => pinyin.replace(vowel, vowels[4][tone_index]),
                        "o" => pinyin.replace(vowel, vowels[5][tone_index]),
                        "u" => pinyin.replace(vowel, vowels[6][tone_index]),
                        "ê" => pinyin.replace(vowel, vowels[7][tone_index]),
                        "ü" => pinyin.replace(vowel, vowels[8][tone_index]),
                        _   => unreachable!(),
                    }
                },
                Style::Number1 => {
                    let vowels = [
                        [ "a", "a1", "a2", "a3", "a4", ],
                        [ "e", "e1", "e2", "e3", "e4", ],
                        [ "i", "i1", "i2", "i3", "i4", ],
                        [ "m", "m1", "m2", "m3", "m4", ],
                        [ "n", "n1", "n2", "n3", "n4", ],
                        [ "o", "o1", "o2", "o3", "o4", ],
                        [ "u", "u1", "u2", "u3", "u4", ],
                        [ "ê", "ê1", "ê2", "ê3", "ê4", ],
                        [ "ü", "ü1", "ü2", "ü3", "ü4", ],
                    ];

                    match vowel {
                        "a" => pinyin.replace(vowel, vowels[0][tone_index]),
                        "e" => pinyin.replace(vowel, vowels[1][tone_index]),
                        "i" => pinyin.replace(vowel, vowels[2][tone_index]),
                        "m" => pinyin.replace(vowel, vowels[3][tone_index]),
                        "n" => pinyin.replace(vowel, vowels[4][tone_index]),
                        "o" => pinyin.replace(vowel, vowels[5][tone_index]),
                        "u" => pinyin.replace(vowel, vowels[6][tone_index]),
                        "ê" => pinyin.replace(vowel, vowels[7][tone_index]),
                        "ü" => pinyin.replace(vowel, vowels[8][tone_index]),
                        _   => unreachable!(),
                    }
                },
                Style::Number2 => {
                    format!("{}{}", pinyin, tone_index)
                },
            };

            print!("{:>width$}, ", format!("\"{}\"", pinyin_with_tone), width = 9);
        }
        
        pos += 1;
    }
    println!();
}

fn rhyme_table_codegen(tone_index: usize, style: Style) {
    print!("    ");
    for rhyme in RHYME_TABLE.iter() {
        let (vowel, _start, _end) = search_vowel(&rhyme);

        let rhyme_with_tone = match style {
            Style::Symbol => {
                let vowels = [
                    [ "a", "ā", "á", "ǎ", "à", ],
                    [ "e", "ē", "é", "ě", "è", ],
                    [ "i", "ī", "í", "ǐ", "ì", ],
                    [ "m", "m̄", "ḿ", "m̌", "m̀", ],
                    [ "n", "n̄", "ń", "ň", "ǹ", ],
                    [ "o", "ō", "ó", "ǒ", "ò", ],
                    [ "u", "ū", "ú", "ǔ", "ù", ],
                    [ "ê", "ê̄", "ế", "ê̌", "ề", ],
                    [ "ü", "ǖ", "ǘ", "ǚ", "ǜ", ],
                ];

                match vowel {
                    "a" => rhyme.replace(vowel, vowels[0][tone_index]),
                    "e" => rhyme.replace(vowel, vowels[1][tone_index]),
                    "i" => rhyme.replace(vowel, vowels[2][tone_index]),
                    "m" => rhyme.replace(vowel, vowels[3][tone_index]),
                    "n" => rhyme.replace(vowel, vowels[4][tone_index]),
                    "o" => rhyme.replace(vowel, vowels[5][tone_index]),
                    "u" => rhyme.replace(vowel, vowels[6][tone_index]),
                    "ê" => rhyme.replace(vowel, vowels[7][tone_index]),
                    "ü" => rhyme.replace(vowel, vowels[8][tone_index]),
                    _   => unreachable!(),
                }
            },
            Style::Number1 => {
                let vowels = [
                    [ "a", "a1", "a2", "a3", "a4", ],
                    [ "e", "e1", "e2", "e3", "e4", ],
                    [ "i", "i1", "i2", "i3", "i4", ],
                    [ "m", "m1", "m2", "m3", "m4", ],
                    [ "n", "n1", "n2", "n3", "n4", ],
                    [ "o", "o1", "o2", "o3", "o4", ],
                    [ "u", "u1", "u2", "u3", "u4", ],
                    [ "ê", "ê1", "ê2", "ê3", "ê4", ],
                    [ "ü", "ü1", "ü2", "ü3", "ü4", ],
                ];

                match vowel {
                    "a" => rhyme.replace(vowel, vowels[0][tone_index]),
                    "e" => rhyme.replace(vowel, vowels[1][tone_index]),
                    "i" => rhyme.replace(vowel, vowels[2][tone_index]),
                    "m" => rhyme.replace(vowel, vowels[3][tone_index]),
                    "n" => rhyme.replace(vowel, vowels[4][tone_index]),
                    "o" => rhyme.replace(vowel, vowels[5][tone_index]),
                    "u" => rhyme.replace(vowel, vowels[6][tone_index]),
                    "ê" => rhyme.replace(vowel, vowels[7][tone_index]),
                    "ü" => rhyme.replace(vowel, vowels[8][tone_index]),
                    _   => unreachable!(),
                }
            },
            Style::Number2 => {
                format!("{}{}", rhyme, tone_index)
            },
        };

        print!("{}, ", format!("\"{}\"", rhyme_with_tone));

        if *rhyme == "ong" || *rhyme == "ng" || *rhyme == "iong" || *rhyme == "ueng" || *rhyme == "ün" {
            print!("\n    ");
        }
    }
    println!();
}

fn main() {
    println!("// 符号音调");
    println!("// 第一声: 音调符号标记形式");
    pinyin_table_codegen(1, Style::Symbol);
    println!("// 第二声: 音调符号标记形式");
    pinyin_table_codegen(2, Style::Symbol);
    println!("// 第三声: 音调符号标记形式");
    pinyin_table_codegen(3, Style::Symbol);
    println!("// 第四声: 音调符号标记形式");
    pinyin_table_codegen(4, Style::Symbol);

    println!("// 数字音调, 数字跟在元音后面");
    println!("// 第一声: 数字音调标记, 数字跟在元音后面");
    pinyin_table_codegen(1, Style::Number1);
    println!("// 第二声: 数字音调标记, 数字跟在元音后面");
    pinyin_table_codegen(2, Style::Number1);
    println!("// 第三声: 数字音调标记, 数字跟在元音后面");
    pinyin_table_codegen(3, Style::Number1);
    println!("// 第四声: 数字音调标记, 数字跟在元音后面");
    pinyin_table_codegen(4, Style::Number1);

    println!("// 数字音调, 数字跟在音节后面");
    println!("// 第一声: 数字音调标记, 数字跟在音节后面");
    pinyin_table_codegen(1, Style::Number2);
    println!("// 第二声: 数字音调标记, 数字跟在音节后面");
    pinyin_table_codegen(2, Style::Number2);
    println!("// 第三声: 数字音调标记, 数字跟在音节后面");
    pinyin_table_codegen(3, Style::Number2);
    println!("// 第四声: 数字音调标记, 数字跟在音节后面");
    pinyin_table_codegen(4, Style::Number2);

    println!("\n\n");
    println!("// 符号音调");
    rhyme_table_codegen(1, Style::Symbol);
    rhyme_table_codegen(2, Style::Symbol);
    rhyme_table_codegen(3, Style::Symbol);
    rhyme_table_codegen(4, Style::Symbol);
    println!("// 数字音调, 数字跟在元音后面");
    rhyme_table_codegen(1, Style::Number1);
    rhyme_table_codegen(2, Style::Number1);
    rhyme_table_codegen(3, Style::Number1);
    rhyme_table_codegen(4, Style::Number1);
    println!("// 数字音调, 数字跟在音节后面");
    rhyme_table_codegen(1, Style::Number2);
    rhyme_table_codegen(2, Style::Number2);
    rhyme_table_codegen(3, Style::Number2);
    rhyme_table_codegen(4, Style::Number2);

    // 元音表
    println!("// 元音表");
    print!("    ");
    for rhyme in RHYME_TABLE.iter() {
        let (vowel, _start, _end) = search_vowel(&rhyme);

        print!("{}, ", format!("b\'{}\'", vowel));

        if *rhyme == "ong" || *rhyme == "ng" || *rhyme == "iong" || *rhyme == "ueng" || *rhyme == "ün" {
            print!("\n    ");
        }
    }

    // 简写韵母表
}