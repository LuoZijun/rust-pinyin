use std::fmt;

use crate::SP;
use crate::tone::{ Tone, ToneFormat, };
use crate::initial::Initial;
use crate::letter::Letter;

const ______: &'static str = SP;


// 韵母表
// 对《汉语拼音》方案当中的韵母表勘误:
//     韵母 `ie` 和 `üe` 实际上应该为 `iê` 和 `üê` ，在书写的时候 如果 韵母 `ê` 不是单独存在的，则写成 `e`
//     遗漏 韵母 `io`
// 
// 补写和改写规则:
//      1. i 列的韵母，前面没有声母的时候，写成 yi（衣）、ya（呀）、ye（耶）、yao（腰）、you（优）、yan（烟）、yin（因）、yang（央）、ying（英）、yong（雍）。
//         注: 在《汉语拼音方案》当中被遗漏了 韵母 `io` ，在没有声母的情况下 改写成 `yo`。
//      2. u 列的韵母，前面没有声母的时候，写成 wu（乌）、wa（蛙）、wo（窝）、wai（歪）、wei（威）、wan（弯）、wen（温）、wang（汪）、weng（翁）。
//      3. ü 列的韵母，前面没有声母的时候，写成 yu（迂）、yue（约）、yuan（冤）、yun（晕）；ü 上两点省略。
//      4. ü 列的韵母跟声母 j、q、x 拼的时候，写成 ju（居）、qu（区）、xu（虚），ü 上两点省略；但是跟声母 l、n 拼的时候，仍然写成 lü（吕）、nü（女）。
//      5. 韵母 `uo` 在跟声母 b、p、m、f 组合时，韵母写成 `o` 。
//      6. iou 、uei 、uen 前面加声母的时候，写成 iu 、 ui 、 un ，例如 niu（牛）、gui（归）、lun（论）。
//      7. 韵母 `i` 要么独立成音节，要么只能和 声母 `zh/ch/sh/r/z/c/s` 结合。
//      8. 韵母ㄦ写成 er，用作韵尾的时候写成 r。例如：“儿童”拼作 ertong，“花儿”拼作 huar 。
//      9. 韵母ㄝ单独使用的时候写成 ê 。
pub static RHYME_TABLE2: [&'static str; 38] = [
           "a",    "o",    "e",    "ê",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en",  "ang",  "eng",  "ong",
//               io 在《汉语拼音方案》当中被遗漏了
   "i",   "ia",   "io",   "ie",                                  "iao",  "iou",  "ian",   "in", "iang",  "ing", "iong",
// yi      ya      yo      ye                                     yao     you     yan     yin   yang    ying    yong
   "u",   "ua",   "uo",                          "uai",  "uei",                  "uan",  "uen", "uang", "ueng",
//  wu     wa      wo                             wai     wei                     wan     wen    wang    weng
   "ü",                   "üe",  "üan",   "ün",
// yu                     yue    yuan     yun     // ü 上两点省略；但是跟声母 l 、 n 拼的时候，仍然写成 lü （吕）、 nü （女）。
];

pub static RHYME_TABLE: [&'static str; 320] = [
// |--------------- 单元音韵母 ----------------| |--------- 复元音韵母 ----------| |----------------- 带鼻音韵母 ------------------|
______,    "a",    "o",    "e",    "ê",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en", ______,  "ang",  "eng", ______,  // 开口呼 ( Group A )
   "i",   "ia", ______, ______,   "ie", ______, ______, ______,  "iao",  "iou",  "ian", ______,   "in", "iang",  "ing", ______,  // 齐齿呼 ( Group I )
   "u",   "ua",   "uo", ______, ______, ______,  "uai",  "uei", ______, ______,  "uan",  "uen", ______, "uang", "ueng",  "ong",  // 合口呼 ( Group U )
   "ü", ______, ______, ______,   "üe", ______, ______, ______, ______, ______,  "üan", ______,   "ün", ______, ______, "iong",  // 撮口呼 ( Group Ü )

// 第一声
______,    "ā",    "ō",    "ē",    "ê̄",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en", ______,  "ang",  "eng", ______,
   "ī",   "ia", ______, ______,   "ie", ______, ______, ______,  "iao",  "iou",  "ian", ______,   "in", "iang",  "ing", ______,
   "ū",   "ua",   "uo", ______, ______, ______,  "uai",  "uei", ______, ______,  "uan",  "uen", ______, "uang", "ueng",  "ong",
   "ǖ", ______, ______, ______,   "üe", ______, ______, ______, ______, ______,  "üan", ______,   "ün", ______, ______, "iong",

// 第二声
______,    "á",    "ó",    "é",    "ế",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en", ______,  "ang",  "eng", ______,
   "í",   "ia", ______, ______,   "ie", ______, ______, ______,  "iao",  "iou",  "ian", ______,   "in", "iang",  "ing", ______,
   "ú",   "ua",   "uo", ______, ______, ______,  "uai",  "uei", ______, ______,  "uan",  "uen", ______, "uang", "ueng",  "ong",
   "ǘ", ______, ______, ______,   "üe", ______, ______, ______, ______, ______,  "üan", ______,   "ün", ______, ______, "iong",

// 第三声
______,    "ǎ",    "ǒ",    "ě",    "ê̌",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en", ______,  "ang",  "eng", ______,
   "ǐ",   "ia", ______, ______,   "ie", ______, ______, ______,  "iao",  "iou",  "ian", ______,   "in", "iang",  "ing", ______,
   "ǔ",   "ua",   "uo", ______, ______, ______,  "uai",  "uei", ______, ______,  "uan",  "uen", ______, "uang", "ueng",  "ong",
   "ǚ", ______, ______, ______,   "üe", ______, ______, ______, ______, ______,  "üan", ______,   "ün", ______, ______, "iong",

// 第四声
______,    "à",    "ò",    "è",    "ề",   "er",   "ai",   "ei",   "ao",   "ou",   "an",   "en", ______,  "ang",  "eng", ______,
   "ì",   "ia", ______, ______,   "ie", ______, ______, ______,  "iao",  "iou",  "ian", ______,   "in", "iang",  "ing", ______,
   "ù",   "ua",   "uo", ______, ______, ______,  "uai",  "uei", ______, ______,  "uan",  "uen", ______, "uang", "ueng",  "ong",
   "ǜ", ______, ______, ______,   "üe", ______, ______, ______, ______, ______,  "üan", ______,   "ün", ______, ______, "iong",
];


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RhymeGroup {
    A,
    I,
    U,
    // Ü
    U2,
}

// 韵母类别
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RhymeCategory {
    // 单元音韵母: SingleVowel
    SingleVowelA,   // 单元音韵母, 开口呼
    SingleVowelI,   // 单元音韵母, 齐齿呼
    SingleVowelU,   // 单元音韵母, 合口呼
    SingleVowelU2,  // 单元音韵母, 撮口呼

    // compound finals
    MultiVowelA,    // 复元音韵母, 开口呼
    MultiVowelI,    // 复元音韵母, 齐齿呼
    MultiVowelU,    // 复元音韵母, 合口呼
    MultiVowelU2,   // 复元音韵母, 撮口呼

    NasalA,         // 鼻音韵母, 开口呼
    NasalI,         // 鼻音韵母, 齐齿呼
    NasalU,         // 鼻音韵母, 合口呼
    NasalU2,        // 鼻音韵母, 撮口呼
}

// 16 Bits
// 低 第0位 - 第3位 被用作指示韵母携带的音调
// 低        第4位 被用作指示韵母是否为简写形式
/// 韵母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rhyme(pub(crate) u16);

impl Rhyme {
    #[inline]
    pub fn offset(&self) -> u16 {
        self.0 >> 4
    }

    #[inline]
    pub fn tone(&self) -> Tone {
        let n = self.0 & 0b0000_0000_0000_0111;
        match n {
            0 => Tone::Neutral,
            1 => Tone::First,
            2 => Tone::Second,
            3 => Tone::Third,
            4 => Tone::Fourth,
            _ => unreachable!(),
        }
    }
    
    #[inline]
    pub fn vowel(&self) -> Letter {
        unimplemented!()
    }

    #[inline]
    pub fn tone_mark(&self) -> Letter {
        unimplemented!()
    }

    #[inline]
    pub fn is_simplified(&self) -> bool {
        (self.0 & 0b0000_0000_0000_1111) >> 3 == 1
    }

    #[inline]
    pub fn plain(&self) -> &'static str {
        let s = RHYME_TABLE[self.0 as usize];
        debug_assert!(s != ______);
        s
    }

    #[inline]
    pub fn category(&self) -> RhymeCategory {
        unimplemented!()
    }

    #[inline]
    pub fn has_single_vowel(&self) -> bool {
        // 单元音 韵母
        match self.0 {
             0 ...  5 => true,
            16 ... 21 => true,
            32 ... 37 => true,
            // NOTE: 鼻音需要加入判断中来吗？
            _ => false,
        }
    }

    #[inline]
    pub fn has_multi_vowel(&self) -> bool {
        // 复元音 韵母
        match self.0 {
             6 ...  9 => true,
            22 ... 25 => true,
            38 ... 41 => true,
            // NOTE: 鼻音需要加入判断中来吗？
            _ => false,
        }
    }

    #[inline]
    pub fn has_nasal(&self) -> bool {
        // 携带鼻音的韵母
        match self.0 {
            10 ... 15 => true,
            26 ... 31 => true,
            42 ... 47 => true,
            _ => false,
        }
    }

    #[inline]
    pub fn medial(&self) -> Option<&'static str> {
        if !self.has_multi_vowel() {
            return None;
        }

        // TOOD:
        unimplemented!()
    }

    // 转换为简写形式
    #[inline]
    pub fn simplified(&self) -> Self {
        if self.is_simplified() {
            *self
        } else {
            Self(self.0 | 0b0000_0000_0000_1000 | (self.0 & 0b0000_0000_0000_0111) )
        }
    }

    // 重新设置音调
    #[inline]
    pub fn with_tone(&self, tone: Tone) -> Self {
        match tone {
            Tone::Neutral => Self(self.0 | 0b0000_0000_0000_0000),
            Tone::First   => Self(self.0 | 0b0000_0000_0000_0001),
            Tone::Second  => Self(self.0 | 0b0000_0000_0000_0010),
            Tone::Third   => Self(self.0 | 0b0000_0000_0000_0011),
            Tone::Fourth  => Self(self.0 | 0b0000_0000_0000_0100),
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        let s = if self.is_simplified() {
            // ng => ŋ
            match self.offset() {
                13 => "aŋ",       // ang
                14 => "eŋ",       // eng
                29 => "iaŋ",      // iang
                30 => "iŋ",       // ing
                45 => "uaŋ",      // uang
                46 => "ueŋ",      // ueng
                47 => "oŋ",       // ong
                63 => "ioŋ",      // iong
                _ => self.plain(),// 原始形式
            }
        } else {
            self.plain()
        };
        
        // 标注声调
        // 汉语拼音中标声调位置的规则如下：
        // 
        // 1. 如果有a，则标在a上。
        // 2. 如果没有a，但有o或e，则标在这两个字母上。这两个字母不会同时出现。
        // 3. 如果也没有o和e，则一定有i、u或ü。如果i和u同时出现，则标在第二个韵母上。
        //    这是特别针对ui和iu而言的（这两个音的实际读音应该是uei和iou）。
        //    如果i和u不同时出现，则标在出现的那个韵母上。
        let tone = self.tone();

        // match s {
        //     "a" => match tone {
        //         Tone::Neutral => "a",
        //         Tone::First   => "ā",
        //         Tone::Second  => "á",
        //     },
        //     "o" => ,
        //     "e" => ,
        //     "ê" => ,
        //     "er" => ,
        //     "ai" => ,
        //     "ei" => ,
        //     "ao" => ,
        //     "ou" => ,
        //     "an" => ,
        //     "en" => , 
        //     "ang" => , 
        //     "eng" => ,

        //     "i" => ,
        //     "ia" => ,
        //     "ie" => ,
        //     "iao" => ,
        //     "iou" => ,
        //     "ian" => ,
        //     "in" => ,
        //     "iang" => ,
        //     "ing" => ,

        //     "u" => ,
        //     "ua" => ,
        //     "uo" => ,
        //     "uai" => ,
        //     "uei" => ,
        //     "uan" => ,
        //     "uen" => , 
        //     "uang" => ,
        //     "ueng" => ,
        //     "ong" => ,

        //     "ü" => ,
        //     "üe" => ,
        //     "üan" => ,
        //     "ün" => ,
        //     "iong" => ,
        // }

        unimplemented!()
    }

    // 格式化输出韵母部分 (注: 这个输出不会应用 补写和改写 规则)
    #[inline]
    pub fn format(&self, fmt: ToneFormat) -> &'static str {
        match fmt {
            ToneFormat::Ignore => {
                self.as_str()
            },
            ToneFormat::Symbol => {
                unimplemented!()
            },
            ToneFormat::Digit => {
                unimplemented!()
            },
            ToneFormat::Index => {
                unimplemented!()
            },
        }
    }

    // 格式化输出韵母部分
    // 注: 这个输出会根据 声母部分来 应用 补写和改写 规则
    #[inline]
    pub fn format_with_initials(&self, _initials: Option<Initial>, fmt: ToneFormat) -> &'static str {
        match fmt {
            ToneFormat::Ignore => {
                self.as_str()
            },
            ToneFormat::Symbol => {
                unimplemented!()
            },
            ToneFormat::Digit => {
                unimplemented!()
            },
            ToneFormat::Index => {
                unimplemented!()
            },
        }
    }
}

impl fmt::Debug for Rhyme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.plain())
    }
}

impl fmt::Display for Rhyme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.plain())
    }
}