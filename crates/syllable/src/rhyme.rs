use core::fmt;

use crate::SP;
use crate::tone::{ Tone, ToneFormat, };
use crate::initial::Initial;
use crate::letter::Letter;

const ______: &'static str = SP;


// 韵母表
#[rustfmt::skip]
pub static RHYME_TABLE: [&'static str; 520] = [
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

// 符号音调
    "ā", "ē", "ê̄", "ēr", "āi", "ēi", "āo", "ōu", "ān", "ēn", "āng", "ēng", "ōng",
    "n̄", "m̄", "n̄g",
    "ī", "iā", "iō", "iē", "iāo", "iōu", "iān", "īn", "iāng", "īng", "iōng",
    "ū", "uā", "uō", "uāi", "uēi", "uān", "uēn", "uāng", "uēng",
    "ǖ", "üē", "üān", "ǖn",

    "á", "é", "ế", "ér", "ái", "éi", "áo", "óu", "án", "én", "áng", "éng", "óng",
    "ń", "ḿ", "ńg",
    "í", "iá", "ió", "ié", "iáo", "ióu", "ián", "ín", "iáng", "íng", "ióng",
    "ú", "uá", "uó", "uái", "uéi", "uán", "uén", "uáng", "uéng",
    "ǘ", "üé", "üán", "ǘn",

    "ǎ", "ě", "ê̌", "ěr", "ǎi", "ěi", "ǎo", "ǒu", "ǎn", "ěn", "ǎng", "ěng", "ǒng",
    "ň", "m̌", "ňg",
    "ǐ", "iǎ", "iǒ", "iě", "iǎo", "iǒu", "iǎn", "ǐn", "iǎng", "ǐng", "iǒng",
    "ǔ", "uǎ", "uǒ", "uǎi", "uěi", "uǎn", "uěn", "uǎng", "uěng",
    "ǚ", "üě", "üǎn", "ǚn",

    "à", "è", "ề", "èr", "ài", "èi", "ào", "òu", "àn", "èn", "àng", "èng", "òng",
    "ǹ", "m̀", "ǹg",
    "ì", "ià", "iò", "iè", "iào", "iòu", "iàn", "ìn", "iàng", "ìng", "iòng",
    "ù", "uà", "uò", "uài", "uèi", "uàn", "uèn", "uàng", "uèng",
    "ǜ", "üè", "üàn", "ǜn",

// 数字音调, 数字跟在元音后面
    "a1", "e1", "ê1", "e1r", "a1i", "e1i", "a1o", "o1u", "a1n", "e1n", "a1ng", "e1ng", "o1ng",
    "n1", "m1", "n1g",
    "i1", "ia1", "io1", "ie1", "ia1o", "io1u", "ia1n", "i1n", "ia1ng", "i1ng", "io1ng",
    "u1", "ua1", "uo1", "ua1i", "ue1i", "ua1n", "ue1n", "ua1ng", "ue1ng",
    "ü1", "üe1", "üa1n", "ü1n",

    "a2", "e2", "ê2", "e2r", "a2i", "e2i", "a2o", "o2u", "a2n", "e2n", "a2ng", "e2ng", "o2ng",
    "n2", "m2", "n2g",
    "i2", "ia2", "io2", "ie2", "ia2o", "io2u", "ia2n", "i2n", "ia2ng", "i2ng", "io2ng",
    "u2", "ua2", "uo2", "ua2i", "ue2i", "ua2n", "ue2n", "ua2ng", "ue2ng",
    "ü2", "üe2", "üa2n", "ü2n",

    "a3", "e3", "ê3", "e3r", "a3i", "e3i", "a3o", "o3u", "a3n", "e3n", "a3ng", "e3ng", "o3ng",
    "n3", "m3", "n3g",
    "i3", "ia3", "io3", "ie3", "ia3o", "io3u", "ia3n", "i3n", "ia3ng", "i3ng", "io3ng",
    "u3", "ua3", "uo3", "ua3i", "ue3i", "ua3n", "ue3n", "ua3ng", "ue3ng",
    "ü3", "üe3", "üa3n", "ü3n",

    "a4", "e4", "ê4", "e4r", "a4i", "e4i", "a4o", "o4u", "a4n", "e4n", "a4ng", "e4ng", "o4ng",
    "n4", "m4", "n4g",
    "i4", "ia4", "io4", "ie4", "ia4o", "io4u", "ia4n", "i4n", "ia4ng", "i4ng", "io4ng",
    "u4", "ua4", "uo4", "ua4i", "ue4i", "ua4n", "ue4n", "ua4ng", "ue4ng",
    "ü4", "üe4", "üa4n", "ü4n",

// 数字音调, 数字跟在音节后面
    "a1", "e1", "ê1", "er1", "ai1", "ei1", "ao1", "ou1", "an1", "en1", "ang1", "eng1", "ong1",
    "n1", "m1", "ng1",
    "i1", "ia1", "io1", "ie1", "iao1", "iou1", "ian1", "in1", "iang1", "ing1", "iong1",
    "u1", "ua1", "uo1", "uai1", "uei1", "uan1", "uen1", "uang1", "ueng1",
    "ü1", "üe1", "üan1", "ün1",

    "a2", "e2", "ê2", "er2", "ai2", "ei2", "ao2", "ou2", "an2", "en2", "ang2", "eng2", "ong2",
    "n2", "m2", "ng2",
    "i2", "ia2", "io2", "ie2", "iao2", "iou2", "ian2", "in2", "iang2", "ing2", "iong2",
    "u2", "ua2", "uo2", "uai2", "uei2", "uan2", "uen2", "uang2", "ueng2",
    "ü2", "üe2", "üan2", "ün2",

    "a3", "e3", "ê3", "er3", "ai3", "ei3", "ao3", "ou3", "an3", "en3", "ang3", "eng3", "ong3",
    "n3", "m3", "ng3",
    "i3", "ia3", "io3", "ie3", "iao3", "iou3", "ian3", "in3", "iang3", "ing3", "iong3",
    "u3", "ua3", "uo3", "uai3", "uei3", "uan3", "uen3", "uang3", "ueng3",
    "ü3", "üe3", "üan3", "ün3",

    "a4", "e4", "ê4", "er4", "ai4", "ei4", "ao4", "ou4", "an4", "en4", "ang4", "eng4", "ong4",
    "n4", "m4", "ng4",
    "i4", "ia4", "io4", "ie4", "iao4", "iou4", "ian4", "in4", "iang4", "ing4", "iong4",
    "u4", "ua4", "uo4", "uai4", "uei4", "uan4", "uen4", "uang4", "ueng4",
    "ü4", "üe4", "üan4", "ün4",
];

// "ang", "eng", "ong",   // 10 ..= 12
// "ng",                  // 15
// "iang", "ing", "iong", // 24 ..= 26
// "uang", "ueng",        // 34 ..= 35
// ŋ ŋ̄ ŋ́ ŋ̌ ŋ̀
// 
// Len: 9 * 13 = 117
#[rustfmt::skip]
pub static SIMPLIFIED_RHYME_TABLE: [&'static str; 117] = [
    // "ang", "eng", "ong",   // 10 ..= 12
    // "ng",                  // 15
    // "iang", "ing", "iong", // 24 ..= 26
    // "uang", "ueng",        // 34 ..= 35
    "aŋ", "eŋ", "oŋ", "ŋ", "iaŋ", "iŋ", "ioŋ", "uaŋ", "ueŋ",
// 符号音调
    "āŋ", "ēŋ", "ōŋ", "ŋ̄", "iāŋ", "īŋ", "iōŋ", "uāŋ", "uēŋ",
    "áŋ", "éŋ", "óŋ", "ŋ́", "iáŋ", "íŋ", "ióŋ", "uáŋ", "uéŋ",
    "ǎŋ", "ěŋ", "ǒŋ", "ŋ̌", "iǎŋ", "ǐŋ", "iǒŋ", "uǎŋ", "uěŋ",
    "àŋ", "èŋ", "òŋ", "ŋ̀", "iàŋ", "ìŋ", "iòŋ", "uàŋ", "uèŋ",
// 数字音调, 数字跟在元音后面
    "a1ŋ", "e1ŋ", "o1ŋ", "n1g", "ia1ŋ", "i1ŋ", "io1ŋ", "ua1ŋ", "ue1ŋ",
    "a2ŋ", "e2ŋ", "o2ŋ", "n2g", "ia2ŋ", "i2ŋ", "io2ŋ", "ua2ŋ", "ue2ŋ",
    "a3ŋ", "e3ŋ", "o3ŋ", "n3g", "ia3ŋ", "i3ŋ", "io3ŋ", "ua3ŋ", "ue3ŋ",
    "a4ŋ", "e4ŋ", "o4ŋ", "n4g", "ia4ŋ", "i4ŋ", "io4ŋ", "ua4ŋ", "ue4ŋ",
// 数字音调, 数字跟在音节后面
    "aŋ1", "eŋ1", "oŋ1", "ŋ1", "iaŋ1", "iŋ1", "ioŋ1", "uaŋ1", "ueŋ1",
    "aŋ2", "eŋ2", "oŋ2", "ŋ2", "iaŋ2", "iŋ2", "ioŋ2", "uaŋ2", "ueŋ2",
    "aŋ3", "eŋ3", "oŋ3", "ŋ3", "iaŋ3", "iŋ3", "ioŋ3", "uaŋ3", "ueŋ3",
    "aŋ4", "eŋ4", "oŋ4", "ŋ4", "iaŋ4", "iŋ4", "ioŋ4", "uaŋ4", "ueŋ4",
];


#[rustfmt::skip]
pub static VOWEL_TABLE: [char; 40] = [
    'a', 'e', 'ê', 'e', 'a', 'e', 'a', 'o', 'a', 'e', 'a', 'e', 'o',
    'n', 'm', 'n',
    'i', 'a', 'o', 'e', 'a', 'o', 'a', 'i', 'a', 'i', 'o',
    'u', 'a', 'o', 'a', 'e', 'a', 'e', 'a', 'e',
    'ü', 'e', 'a', 'ü',
];


// const PLAIN_ROWS: u16   = 46; // 46 * 22 = 1012
const PLAIN_LEN: u16    = 40;
const SIMPLIFIED_RHYME_LEN: u16 = 9;


// 16 Bits
// 低 第1位 被用作指示韵母是否为简写形式
/// 韵母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Rhyme(pub(crate) u16);

impl Rhyme {
    pub const I: Self            = Rhyme(16 << 1); // i
    pub const E_CIRCUMFLEX: Self = Rhyme( 2 << 1); // ê
    pub const U_UMLAUT: Self     = Rhyme(36 << 1); // ü

    // 韵母表偏移量
    #[inline]
    pub fn offset(&self) -> u16 {
        self.0 >> 1
    }

    // 声调
    #[inline]
    pub fn tone(&self) -> Tone {
        match self.offset() / PLAIN_LEN {
            0 => Tone::Neutral,
            
            1 => Tone::First,
            2 => Tone::Second,
            3 => Tone::Third,
            4 => Tone::Fourth,

            5 => Tone::First,
            6 => Tone::Second,
            7 => Tone::Third,
            8 => Tone::Fourth,

            9 => Tone::First,
            10 => Tone::Second,
            11 => Tone::Third,
            12 => Tone::Fourth,

            _ => unreachable!(),
        }
    }
    
    // 当前拼音音调标记格式
    #[inline]
    pub fn tone_format(&self) -> ToneFormat {
        match self.offset() / PLAIN_LEN {
            0 => ToneFormat::Plain,

            1 => ToneFormat::Mark,
            2 => ToneFormat::Mark,
            3 => ToneFormat::Mark,
            4 => ToneFormat::Mark,

            5 => ToneFormat::Number,
            6 => ToneFormat::Number,
            7 => ToneFormat::Number,
            8 => ToneFormat::Number,

            9 => ToneFormat::Number2,
            10 => ToneFormat::Number2,
            11 => ToneFormat::Number2,
            12 => ToneFormat::Number2,

            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn vowel(&self) -> char {
        VOWEL_TABLE[self.offset() as usize]
    }

    #[inline]
    pub fn tone_mark(&self) -> Letter {
        let _vowel = self.vowel();
        let _tone = self.tone();

        unimplemented!()
    }

    #[inline]
    pub fn is_simplified(&self) -> bool {
        self.0 & 0b0000_0000_0000_0001 == 1
    }

    #[inline]
    pub fn plain(&self) -> Self {
        let offset = self.offset();

        let m = offset / PLAIN_LEN;
        debug_assert!(m <= 12);

        let idx = offset - PLAIN_LEN * m;

        Self(idx << 1)
    }

    #[inline]
    pub fn category(&self) -> () {
        unimplemented!()
    }

    #[inline]
    pub fn has_single_vowel(&self) -> bool {
        // 单元音 韵母
        unimplemented!()
    }

    #[inline]
    pub fn has_multi_vowel(&self) -> bool {
        // 复元音 韵母
        unimplemented!()
    }

    #[inline]
    pub fn has_nasal(&self) -> bool {
        // 携带鼻音的韵母
        unimplemented!()
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
            Self(self.0 | 0b0000_0000_0000_0001 )
        }
    }

    // 重新设置音调
    #[inline]
    pub fn with_tone(&self, tone: Tone) -> Self {
        let tone_format = self.tone_format();

        self.format(tone, tone_format)
    }
    
    // 重新设置音调的标记形式
    #[inline]
    pub fn with_tone_fotmat(&self, tone_format: ToneFormat) -> Self {
        let tone = self.tone();

        self.format(tone, tone_format)
    }

    // 格式化输出韵母部分 (注: 这个输出不会应用 补写和改写 规则)
    #[inline]
    pub fn format(&self, tone: Tone, tone_format: ToneFormat) -> Self {
        use crate::tone::ToneFormat::*;
        use crate::tone::Tone::*;

        let plain = self.plain();
        let plain_offset = plain.offset();

        let offset = match tone_format {
            Plain => plain_offset,
            Mark => {
                match tone {
                    Neutral => plain_offset,
                    First   => plain_offset + PLAIN_LEN * 1,
                    Second  => plain_offset + PLAIN_LEN * 2,
                    Third   => plain_offset + PLAIN_LEN * 3,
                    Fourth  => plain_offset + PLAIN_LEN * 4,
                }
            },
            Number => {
                match tone {
                    Neutral => plain_offset,
                    First   => plain_offset + PLAIN_LEN * 5,
                    Second  => plain_offset + PLAIN_LEN * 6,
                    Third   => plain_offset + PLAIN_LEN * 7,
                    Fourth  => plain_offset + PLAIN_LEN * 8,
                }
            },
            Number2 => {
                match tone {
                    Neutral => plain_offset,
                    First   => plain_offset + PLAIN_LEN * 9,
                    Second  => plain_offset + PLAIN_LEN * 10,
                    Third   => plain_offset + PLAIN_LEN * 11,
                    Fourth  => plain_offset + PLAIN_LEN * 12,
                }
            }
        };

        if self.is_simplified() {
            Self(offset << 1).simplified()
        } else {
            Self(offset << 1)
        }
    }

    // 格式化输出韵母部分
    // 注: 这个输出会根据 声母部分来 应用 补写和改写 规则
    #[inline]
    pub fn format_with_initials(&self, _initials: Option<Initial>, _fmt: ToneFormat) -> &'static str {
        unimplemented!()
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        let offset = self.offset();
        let s = RHYME_TABLE[offset as usize];
        debug_assert!(s != ______);

        if self.is_simplified() {
            // "ang", "eng", "ong",   // 10 ..= 12
            // "ng",                  // 15
            // "iang", "ing", "iong", // 24 ..= 26
            // "uang", "ueng",        // 34 ..= 35
            let m = offset / PLAIN_LEN;

            match offset - m * PLAIN_LEN {
                10 => SIMPLIFIED_RHYME_TABLE[(0 + SIMPLIFIED_RHYME_LEN * m) as usize],
                11 => SIMPLIFIED_RHYME_TABLE[(1 + SIMPLIFIED_RHYME_LEN * m) as usize],
                12 => SIMPLIFIED_RHYME_TABLE[(2 + SIMPLIFIED_RHYME_LEN * m) as usize],

                15 => SIMPLIFIED_RHYME_TABLE[(3 + SIMPLIFIED_RHYME_LEN * m) as usize],

                24 => SIMPLIFIED_RHYME_TABLE[(4 + SIMPLIFIED_RHYME_LEN * m) as usize],
                25 => SIMPLIFIED_RHYME_TABLE[(5 + SIMPLIFIED_RHYME_LEN * m) as usize],
                26 => SIMPLIFIED_RHYME_TABLE[(6 + SIMPLIFIED_RHYME_LEN * m) as usize],

                34 => SIMPLIFIED_RHYME_TABLE[(7 + SIMPLIFIED_RHYME_LEN * m) as usize],
                35 => SIMPLIFIED_RHYME_TABLE[(8 + SIMPLIFIED_RHYME_LEN * m) as usize],
                _  => s,
            }
        } else {
            s
        }
    }
}

impl fmt::Debug for Rhyme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl fmt::Display for Rhyme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[test]
fn test_rhyme_const() {
    assert_eq!(Rhyme::I.plain().as_str(), "i");
    assert_eq!(Rhyme::E_CIRCUMFLEX.plain().as_str(), "ê");
    assert_eq!(Rhyme::U_UMLAUT.plain().as_str(), "ü");
}