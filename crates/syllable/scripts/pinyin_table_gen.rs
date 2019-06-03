

pub(crate) const ________: &'static str = " ";

// ∅ b p m f d t n l g k h j q x zh ch sh r z c s
pub static PINYIN_TABLE: [&'static str; 924] = [
//       ∅,        b,        p,        m,        f,        d,        t,        n,        l,        g,        k,        h,        j,        q,        x,       zh,       ch,       sh,        r,        z,        c,        s,
// Group a Finals
  ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,    "zhi",    "chi",    "shi",     "ri",     "zi",     "ci",     "si",
       "a",     "ba",     "pa",     "ma",     "fa",     "da",     "ta",     "na",     "la",     "ga",     "ka",     "ha", ________, ________, ________,    "zha",    "cha",    "sha", ________,     "za",     "ca",     "sa",
       "e", ________, ________,     "me", ________,     "de",     "te",     "ne",     "le",     "ge",     "ke",     "he", ________, ________, ________,    "zhe",    "che",    "she",     "re",     "ze",     "ce",     "se",
       "ê", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
      "er", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
      "ai",    "bai",    "pai",    "mai", ________,    "dai",    "tai",    "nai",    "lai",    "gai",    "kai",    "hai", ________, ________, ________,   "zhai",   "chai",   "shai", ________,    "zai",    "cai",    "sai",
      "ei",    "bei",    "pei",    "mei",    "fei",    "dei",    "tei",    "nei",    "lei",    "gei",    "kei",    "hei", ________, ________, ________,   "zhei", ________,   "shei", ________,    "zei", ________,    "sei",
      "ao",    "bao",    "pao",    "mao", ________,    "dao",    "tao",    "nao",    "lao",    "gao",    "kao",    "hao", ________, ________, ________,   "zhao",   "chao",   "shao",    "rao",    "zao",    "cao",    "sao",
      "ou", ________,    "pou",    "mou",    "fou",    "dou",    "tou",    "nou",    "lou",    "gou",    "kou",    "hou", ________, ________, ________,   "zhou",   "chou",   "shou",    "rou",    "zou",    "cou",    "sou",
      "an",    "ban",    "pan",    "man",    "fan",    "dan",    "tan",    "nan",    "lan",    "gan",    "kan",    "han", ________, ________, ________,   "zhan",   "chan",   "shan",    "ran",    "zan",    "can",    "san",
      "en",    "ben",    "pen",    "men",    "fen",    "den", ________,    "nen", ________,    "gen",    "ken",    "hen", ________, ________, ________,   "zhen",   "chen",   "shen",    "ren",    "zen",    "cen",    "sen",
     "ang",   "bang",   "pang",   "mang",   "fang",   "dang",   "tang",   "nang",   "lang",   "gang",   "kang",   "hang", ________, ________, ________,  "zhang",  "chang",  "shang",   "rang",   "zang",   "cang",   "sang",
     "eng",   "beng",   "peng",   "meng",   "feng",   "deng",   "teng",   "neng",   "leng",   "geng",   "keng",   "heng", ________, ________, ________,  "zheng",  "cheng",  "sheng",   "reng",   "zeng",   "ceng",   "seng",
     // 注: "ong" 不存在

// Group i Finals 
      "yi",     "bi",     "pi",     "mi", ________,     "di",     "ti",     "ni",     "li", ________, ________, ________,     "ji",     "qi",     "xi", ________, ________, ________, ________, ________, ________, ________,
      "ya", ________, ________, ________, ________,    "dia", ________,    "nia",    "lia", ________, ________, ________,    "jia",    "qia",    "xia", ________, ________, ________, ________, ________, ________, ________,
      "yo", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
      "ye",    "bie",    "pie",    "mie", ________,    "die",    "tie",    "nie",    "lie", ________, ________, ________,    "jie",    "qie",    "xie", ________, ________, ________, ________, ________, ________, ________,
     // "yai", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
     "yao",   "biao",   "piao",   "miao",   "fiao",   "diao",   "tiao",   "niao",   "liao", ________, ________, ________,   "jiao",   "qiao",   "xiao", ________, ________, ________, ________, ________, ________, ________,
     "you", ________, ________,    "miu", ________,    "diu", ________,    "niu",    "liu", ________, ________, ________,    "jiu",    "qiu",    "xiu", ________, ________, ________, ________, ________, ________, ________,
     "yan",   "bian",   "pian",   "mian", ________,   "dian",   "tian",   "nian",   "lian", ________, ________, ________,   "jian",   "qian",   "xian", ________, ________, ________, ________, ________, ________, ________,
     "yin",    "bin",    "pin",    "min", ________, ________, ________,    "nin",    "lin", ________, ________, ________,    "jin",    "qin",    "xin", ________, ________, ________, ________, ________, ________, ________,
    "yang",  "biang", ________, ________, ________,  "diang", ________,  "niang",  "liang", ________, ________, ________,  "jiang",  "qiang",  "xiang", ________, ________, ________, ________, ________, ________, ________,
    "ying",   "bing",   "ping",   "ming", ________,   "ding",   "ting",   "ning",   "ling", ________, ________, ________,   "jing",   "qing",   "xing", ________, ________, ________, ________, ________, ________, ________,
    "yong", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,  "jiong",  "qiong",  "xiong", ________, ________, ________, ________, ________, ________, ________,

// Group u Finals
      "wu",     "bu",     "pu",     "mu",     "fu",     "du",     "tu",     "nu",     "lu",     "gu",     "ku",     "hu", ________, ________, ________,    "zhu",    "chu",    "shu",     "ru",     "zu",     "cu",     "su",
      "wa", ________, ________, ________, ________, ________, ________, ________, ________,    "gua",    "kua",    "hua", ________, ________, ________,   "zhua",   "chua",   "shua",    "rua", ________, ________, ________,
      "wo",     "bo",     "po",     "mo",     "fo",    "duo",    "tuo",    "nuo",    "luo",    "guo",    "kuo",    "huo", ________, ________, ________,   "zhuo",   "chuo",   "shuo",    "ruo",    "zuo",    "cuo",    "suo",
     "wai", ________, ________, ________, ________, ________, ________, ________, ________,   "guai",   "kuai",   "huai", ________, ________, ________,  "zhuai",  "chuai",  "shuai", ________, ________, ________, ________,
     "wei", ________, ________, ________, ________,    "dui",    "tui", ________, ________,    "gui",    "kui",    "hui", ________, ________, ________,   "zhui",   "chui",   "shui",    "rui",    "zui",    "cui",    "sui",
     "wan", ________, ________, ________, ________,   "duan",   "tuan",   "nuan",   "luan",   "guan",   "kuan",   "huan", ________, ________, ________,  "zhuan",  "chuan",  "shuan",   "ruan",   "zuan",   "cuan",   "suan",
     "wen", ________, ________, ________, ________,    "dun",    "tun",    "nun",    "lun",    "gun",    "kun",    "hun", ________, ________, ________,   "zhun",   "chun",   "shun",    "run",    "zun",    "cun",    "sun",
    "wang", ________, ________, ________, ________, ________, ________, ________, ________,  "guang",  "kuang",  "huang", ________, ________, ________, "zhuang", "chuang", "shuang", ________, ________, ________, ________,
    "weng", ________, ________, ________, ________,   "dong",   "tong",   "nong",   "long",   "gong",   "kong",   "hong", ________, ________, ________,  "zhong",  "chong",  "shong",   "rong",   "zong",   "cong",   "song",

// Group ü Finals
      "yu", ________, ________, ________, ________, ________, ________,     "nü",     "lü", ________, ________, ________,     "ju",     "qu",     "xu", ________, ________, ________, ________, ________, ________, ________,
     "yue", ________, ________, ________, ________, ________, ________,    "nüe",    "lüe", ________, ________, ________,    "jue",    "que",    "xue", ________, ________, ________, ________, ________, ________, ________,
    "yuan", ________, ________, ________, ________, ________, ________, ________,   "lüan", ________, ________, ________,   "juan",   "quan",   "xuan", ________, ________, ________, ________, ________, ________, ________,
     "yun", ________, ________, ________, ________, ________, ________, ________,    "lün", ________, ________, ________,    "jun",    "qun",    "xun", ________, ________, ________, ________, ________, ________, ________,

// 儿化音节 (er)    
     "wor",   "banr",    "pir",  "mianr",    "fur",  "dianr",  "tangr",    "nar", ________,    "ger",   "kour",   "hair",   "jinr", ________,   "xiar",   "zher", ________,   "shir", ________, ________, ________, ________,
    "wanr", ________, ________, ________, ________,  "dingr",   "tuir",    "nür", ________,   "ganr",  "kongr",   "haor", ________, ________,  "xianr", ________, ________,  "shuir", ________, ________, ________, ________,
  ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,   "huar", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
  ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,   "huor", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,
  ________, ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,   "huir", ________, ________, ________, ________, ________, ________, ________, ________, ________, ________,

];

// 汉语拼音中标声调位置的规则如下：
// 
// 1. 如果有a，则标在a上。
// 2. 如果没有a，但有o或e，则标在这两个字母上。这两个字母不会同时出现。
// 3. 如果也没有o和e，则一定有i、u或ü。如果i和u同时出现，则标在第二个韵母上。
//    这是特别针对ui和iu而言的（这两个音的实际读音应该是uei和iou）。
//    如果i和u不同时出现，则标在出现的那个韵母上。
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

fn mark(s: &str, ch_start: usize, ch_end: usize, tone: usize) -> String {
    // vowel
    // let a = [ "a", "ā", "á", "ǎ", "à", ];
    // let e = [ "e", "ē", "é", "ě", "è", ];
    // let i = [ "i", "ī", "í", "ǐ", "ì", ];
    // let m = [ "m", "m̄", "ḿ", "m̌", "m̀", ];
    // let n = [ "n", "n̄", "ń", "ň", "ǹ", ];
    // let o = [ "o", "ō", "ó", "ǒ", "ò", ];
    // let u = [ "u", "ū", "ú", "ǔ", "ù", ];
    // let e2 = [ "ê", "ê̄", "ế", "ê̌", "ề", ];
    // let u2 = [ "ü", "ǖ", "ǘ", "ǚ", "ǜ", ];

    let a = [ "a", "a1", "a2", "a3", "a4", ];
    let e = [ "e", "e1", "e2", "e3", "e4", ];
    let i = [ "i", "i1", "i2", "i3", "i4", ];
    let m = [ "m", "m1", "m2", "m3", "m4", ];
    let n = [ "n", "n1", "n2", "n3", "n4", ];
    let o = [ "o", "o1", "o2", "o3", "o4", ];
    let u = [ "u", "ū", "ú", "ǔ", "ù", ];
    let e2 = [ "ê", "ê1", "ê2", "ê3", "ê4", ];
    let u2 = [ "ü", "ü1", "ü2", "ü3", "ü4", ];

    let ch = &s[ch_start..ch_end];
    match ch {
        "a" => s.replace(ch, a[tone]),
        "e" => s.replace(ch, e[tone]),
        "i" => s.replace(ch, i[tone]),
        "m" => s.replace(ch, m[tone]),
        "n" => s.replace(ch, n[tone]),
        "o" => s.replace(ch, o[tone]),
        "u" => s.replace(ch, u[tone]),
        "ê" => s.replace(ch, e2[tone]),
        "ü" => s.replace(ch, u2[tone]),
        _ => unreachable!(),
    }
}


fn codegen(tone_index: usize, style: [ [&'static str; 5]; 9]) {
    let mut pos = 0usize;
    for py in PINYIN_TABLE.iter() {
        if pos == 0 {
            print!("  ");
        }

        if pos != 0 && pos % 22 == 0 {
            print!("\n  ");
        }

        if py == &________ {
            print!("________, ");
        } else {
            let a_index = py.find("a");
            let o_index = py.find("o");
            let e_index = py.find("e");

            let i_index = py.find("i");
            let u_index = py.find("u");
            let u2_index = py.find("ü");

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
                        if let Some(index) = py.find("ê") {
                            (index, 2)
                        } else {
                            println!(" {:?} unreachable ...", py);
                            unreachable!()
                        }
                    }
                }
            };

            let start = index;
            let end = index + size;

            // vowel
            let ch = &py[start..end];
            let pinyin_with_tone = match ch {
                "a" => py.replace(ch, style[0][tone_index]),
                "e" => py.replace(ch, style[1][tone_index]),
                "i" => py.replace(ch, style[2][tone_index]),
                "m" => py.replace(ch, style[3][tone_index]),
                "n" => py.replace(ch, style[4][tone_index]),
                "o" => py.replace(ch, style[5][tone_index]),
                "u" => py.replace(ch, style[6][tone_index]),
                "ê" => py.replace(ch, style[7][tone_index]),
                "ü" => py.replace(ch, style[8][tone_index]),
                _ => unreachable!(),
            };
            print!("{:>8}, ", format!("\"{}\"", pinyin_with_tone));
        }
        
        pos += 1;
    }
    println!();
}

fn main() {
    let styles = [
        [
            [ "a", "ā", "á", "ǎ", "à", ],
            [ "e", "ē", "é", "ě", "è", ],
            [ "i", "ī", "í", "ǐ", "ì", ],
            [ "m", "m̄", "ḿ", "m̌", "m̀", ],
            [ "n", "n̄", "ń", "ň", "ǹ", ],
            [ "o", "ō", "ó", "ǒ", "ò", ],
            [ "u", "ū", "ú", "ǔ", "ù", ],
            [ "ê", "ê̄", "ế", "ê̌", "ề", ],
            [ "ü", "ǖ", "ǘ", "ǚ", "ǜ", ],
        ],
        [
            [ "a", "a1", "a2", "a3", "a4", ],
            [ "e", "e1", "e2", "e3", "e4", ],
            [ "i", "i1", "i2", "i3", "i4", ],
            [ "m", "m1", "m2", "m3", "m4", ],
            [ "n", "n1", "n2", "n3", "n4", ],
            [ "o", "o1", "o2", "o3", "o4", ],
            [ "u", "u1", "u2", "u3", "u4", ],
            [ "ê", "ê1", "ê2", "ê3", "ê4", ],
            [ "ü", "ü1", "ü2", "ü3", "ü4", ],
        ],
    ];

    println!("// 符号音调");
    println!("// 第一声: 音调符号标记形式");
    codegen(1, styles[0]);
    println!("// 第二声: 音调符号标记形式");
    codegen(2, styles[0]);
    println!("// 第三声: 音调符号标记形式");
    codegen(3, styles[0]);
    println!("// 第四声: 音调符号标记形式");
    codegen(4, styles[0]);

    println!("// 数字音调");
    println!("// 第一声: 数字音调标记");
    codegen(1, styles[1]);
    println!("// 第二声: 数字音调标记");
    codegen(2, styles[1]);
    println!("// 第三声: 数字音调标记");
    codegen(3, styles[1]);
    println!("// 第四声: 数字音调标记");
    codegen(4, styles[1]);
}