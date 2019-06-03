use std::fmt;


// 字符 :
//      Unicode 码位为: U+E7C8 
//      现已被 U+01F9 替代
// 字符 :
//      Unicode 码位为: U+E7C7
//      现已被 U+1E3F 替代
// 在处理源码字符串时，请务必先解决这些遗留问题。
// 
// 字符 "" 这个字符早先出现在 GBK 编码当中，码位为: 0xA8BC
// 被 GB 18030-2000 收录时 映射为: U+E7C7
// 被 GB 18030-2005 收录时 映射为: U+1E3F
// 所以，在处理 Unicode 时，需要把 `U+E7C7` 替换成最新的映射码位: U+1E3F
// 参考:
//      http://code.web.idv.hk/gb18030/gb18030.php?i=1
//      http://www.khngai.com/chinese/charmap/tblgbk.php?page=2
// 
// Unicode 修饰字符
// 第一声: \u0304  ̄
// 第二声: \u0301  ́
// 第三声: \u030C  ̌
// 第四声: \u0300  ̀


// 上标数字: ⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾ⁿⁱ
// https://unicode-table.com/cn/blocks/superscripts-and-subscripts/
pub static SUPER_SCRIPT_TABLE: [&'static str; 10] = [
    "⁰", "¹", "²", "³", "⁴", 
    "⁵", "⁶", "⁷", "⁸", "⁹",
];

// 扬抑符 e-circumflex: ê Ê
// 分音符 u-diaeresis:  ü Ü
// 非自造组合字符: "m̀" "M̀" "ê̄" "ê̌" "Ê̄" "Ê̌"
// 自造字符: 6 个 ( "m̄" "m̌" "n̄" "M̄" "M̌" "N̄" )
// 
/// 拼音字母表
pub static LETTER_TABLE: [&'static str; 166] = [
    // 声母表码位段
    "b", "p", "m", "f", "d",
    "t", "n", "l", "g", "k",
    "h", "j", "q", "x", "r",
    "z", "c", "s",
                   "w", "y", // 补写字母
    "v", "ẑ", "ĉ", "ŝ", "ŋ", // 特殊字母以及简写字母
    // 元音字母码位段
    "a", "ā", "á", "ǎ", "à",
    "e", "ē", "é", "ě", "è",
    "i", "ī", "í", "ǐ", "ì",
    "m", "m̄", "ḿ", "m̌", "m̀",
    "n", "n̄", "ń", "ň", "ǹ",
    "o", "ō", "ó", "ǒ", "ò",
    "u", "ū", "ú", "ǔ", "ù",
    "ê", "ê̄", "ế", "ê̌", "ề",
    "ü", "ǖ", "ǘ", "ǚ", "ǜ",

    // 大写字母
    "B", "P", "M", "F", "D",
    "T", "N", "L", "G", "K",
    "H", "J", "Q", "X", "R",
    "Z", "C", "S",
                   "W", "Y",
    "V", "Ẑ", "Ĉ", "Ŝ", "Ŋ",
    "A", "Ā", "Á", "Ǎ", "À",
    "E", "Ē", "É", "Ě", "È",
    "I", "Ī", "Í", "Ǐ", "Ì",
    "M", "M̄", "Ḿ", "M̌", "M̀",
    "N", "N̄", "Ń", "Ň", "Ǹ",
    "O", "Ō", "Ó", "Ǒ", "Ò",
    "U", "Ū", "Ú", "Ǔ", "Ù",
    "Ê", "Ê̄", "Ế", "Ê̌", "Ề",
    "Ü", "Ǖ", "Ǘ", "Ǚ", "Ǜ",

    "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
    "⁰", "¹", "²", "³", "⁴",
    "⁵", "⁶", "⁷", "⁸", "⁹",
    // 音调符号
    "˙", "ˉ", "ˊ", "ˇ", "ˋ",
    // 隔音符号
    "'",
];


// 8 Bits
// 拼音字母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Letter(pub(crate) u8);

impl Letter {
    pub const MIN: Self = Letter(0);
    pub const MAX: Self = Letter(166);

    #[inline]
    pub fn as_str(&self) -> &'static str {
        LETTER_TABLE[self.0 as usize]
    }
}

impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<u8> for Letter {
    fn into(self) -> u8 {
        self.0
    }
}

impl Letter {
    pub fn len_utf8(self) -> usize {
        unimplemented!()
    }

    pub fn len_utf16(self) -> usize {
        unimplemented!()
    }

    pub fn len_utf32(self) -> usize {
        unimplemented!()
    }

    pub fn len_gb18030(self) -> usize {
        unimplemented!()
    }

    pub fn encode_utf8(self, _dst: &mut [u8]) -> usize {
        unimplemented!()
    }

    pub fn encode_utf16(self, _dst: &mut [u16]) -> usize {
        unimplemented!()
    }

    pub fn encode_utf32(self, _dst: &mut [u32]) -> usize {
        unimplemented!()
    }

    pub fn encode_gb18030(self, _dst: &mut [u8]) -> usize {
        unimplemented!()
    }

    #[inline]
    pub fn is_lowercase(self) -> bool {
        // 5 * 14 = 70
        match self.0 {
             0 ..=  69 => true,
            70 ..= 139 => false,
            _          => false,
        }
    }

    #[inline]
    pub fn is_uppercase(self) -> bool {
        match self.0 {
             0 ..=  69 => false,
            70 ..= 139 => true,
            _          => false,
        }
    }

    #[inline]
    pub fn is_simplified(&self) -> bool {
        // "ẑ", "ĉ", "ŝ", "ŋ"
        // 21    22   23  24
        // "Ẑ", "Ĉ", "Ŝ", "Ŋ",
        // 96    97   98   99
        match self.0 {
            21 ..= 24 => true,
            96 ..= 99 => true,
            _         => false,
        }
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        !self.is_simplified()
    }

    #[inline]
    pub fn to_lowercase(&self) -> Self {
        match self.0 {
             0 ..=  69 => *self,
            70 ..= 139 => Self(self.0 - 70),
            _          => *self,
        }
    }

    #[inline]
    pub fn to_uppercase(&self) -> Self {
        match self.0 {
             0 ..=  69 => Self(self.0 + 70),
            70 ..= 139 => *self,
            _          => *self,
        }
    }

    pub fn is_id_start(self) -> bool {
        unimplemented!()
    }

    pub fn is_id_continue(self) -> bool {
        unimplemented!()
    }

    pub fn is_xid_start(self) -> bool {
        unimplemented!()
    }

    pub fn is_xid_continue(self) -> bool {
        unimplemented!()
    }

    // 包括 `w/W/y/Y`
    pub fn is_initial(self) -> bool {
        match self.0 {
             0 ..= 19 => true,
            21 ..= 23 => true, // "ẑ", "ĉ", "ŝ"
            70 ..= 89 => true,
            96 ..= 98 => true, // "Ẑ", "Ĉ", "Ŝ"
            _         => false,
        }
    }

    pub fn is_vowel(self) -> bool {
        match self.0 {
            25 ..=  69 => true,
            95 ..= 139 => true,
            _         => false,
        }
    }
}


impl std::str::FromStr for Letter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(Letter(0)),
            "p" => Ok(Letter(1)),
            "m" => Ok(Letter(2)),
            "f" => Ok(Letter(3)),
            "d" => Ok(Letter(4)),
            "t" => Ok(Letter(5)),
            "n" => Ok(Letter(6)),
            "l" => Ok(Letter(7)),
            "g" => Ok(Letter(8)),
            "k" => Ok(Letter(9)),
            "h" => Ok(Letter(10)),
            "j" => Ok(Letter(11)),
            "q" => Ok(Letter(12)),
            "x" => Ok(Letter(13)),
            "r" => Ok(Letter(14)),
            "z" => Ok(Letter(15)),
            "c" => Ok(Letter(16)),
            "s" => Ok(Letter(17)),
            "w" => Ok(Letter(18)),
            "y" => Ok(Letter(19)),
            "v" => Ok(Letter(20)),
            "ẑ" => Ok(Letter(21)),
            "ĉ" => Ok(Letter(22)),
            "ŝ" => Ok(Letter(23)),
            "ŋ" => Ok(Letter(24)),
            "a" => Ok(Letter(25)),
            "ā" => Ok(Letter(26)),
            "á" => Ok(Letter(27)),
            "ǎ" => Ok(Letter(28)),
            "à" => Ok(Letter(29)),
            "e" => Ok(Letter(30)),
            "ē" => Ok(Letter(31)),
            "é" => Ok(Letter(32)),
            "ě" => Ok(Letter(33)),
            "è" => Ok(Letter(34)),
            "i" => Ok(Letter(35)),
            "ī" => Ok(Letter(36)),
            "í" => Ok(Letter(37)),
            "ǐ" => Ok(Letter(38)),
            "ì" => Ok(Letter(39)),
            "m" => Ok(Letter(40)),
            "m̄" => Ok(Letter(41)),
            "ḿ" => Ok(Letter(42)),
            "m̌" => Ok(Letter(43)),
            "m̀" => Ok(Letter(44)),
            "n" => Ok(Letter(45)),
            "n̄" => Ok(Letter(46)),
            "ń" => Ok(Letter(47)),
            "ň" => Ok(Letter(48)),
            "ǹ" => Ok(Letter(49)),
            "o" => Ok(Letter(50)),
            "ō" => Ok(Letter(51)),
            "ó" => Ok(Letter(52)),
            "ǒ" => Ok(Letter(53)),
            "ò" => Ok(Letter(54)),
            "u" => Ok(Letter(55)),
            "ū" => Ok(Letter(56)),
            "ú" => Ok(Letter(57)),
            "ǔ" => Ok(Letter(58)),
            "ù" => Ok(Letter(59)),
            "ê" => Ok(Letter(60)),
            "ê̄" => Ok(Letter(61)),
            "ế" => Ok(Letter(62)),
            "ê̌" => Ok(Letter(63)),
            "ề" => Ok(Letter(64)),
            "ü" => Ok(Letter(65)),
            "ǖ" => Ok(Letter(66)),
            "ǘ" => Ok(Letter(67)),
            "ǚ" => Ok(Letter(68)),
            "ǜ" => Ok(Letter(69)),
            "B" => Ok(Letter(70)),
            "P" => Ok(Letter(71)),
            "M" => Ok(Letter(72)),
            "F" => Ok(Letter(73)),
            "D" => Ok(Letter(74)),
            "T" => Ok(Letter(75)),
            "N" => Ok(Letter(76)),
            "L" => Ok(Letter(77)),
            "G" => Ok(Letter(78)),
            "K" => Ok(Letter(79)),
            "H" => Ok(Letter(80)),
            "J" => Ok(Letter(81)),
            "Q" => Ok(Letter(82)),
            "X" => Ok(Letter(83)),
            "R" => Ok(Letter(84)),
            "Z" => Ok(Letter(85)),
            "C" => Ok(Letter(86)),
            "S" => Ok(Letter(87)),
            "W" => Ok(Letter(88)),
            "Y" => Ok(Letter(89)),
            "V" => Ok(Letter(90)),
            "Ẑ" => Ok(Letter(91)),
            "Ĉ" => Ok(Letter(92)),
            "Ŝ" => Ok(Letter(93)),
            "Ŋ" => Ok(Letter(94)),
            "A" => Ok(Letter(95)),
            "Ā" => Ok(Letter(96)),
            "Á" => Ok(Letter(97)),
            "Ǎ" => Ok(Letter(98)),
            "À" => Ok(Letter(99)),
            "E" => Ok(Letter(100)),
            "Ē" => Ok(Letter(101)),
            "É" => Ok(Letter(102)),
            "Ě" => Ok(Letter(103)),
            "È" => Ok(Letter(104)),
            "I" => Ok(Letter(105)),
            "Ī" => Ok(Letter(106)),
            "Í" => Ok(Letter(107)),
            "Ǐ" => Ok(Letter(108)),
            "Ì" => Ok(Letter(109)),
            "M" => Ok(Letter(110)),
            "M̄" => Ok(Letter(111)),
            "Ḿ" => Ok(Letter(112)),
            "M̌" => Ok(Letter(113)),
            "M̀" => Ok(Letter(114)),
            "N" => Ok(Letter(115)),
            "N̄" => Ok(Letter(116)),
            "Ń" => Ok(Letter(117)),
            "Ň" => Ok(Letter(118)),
            "Ǹ" => Ok(Letter(119)),
            "O" => Ok(Letter(120)),
            "Ō" => Ok(Letter(121)),
            "Ó" => Ok(Letter(122)),
            "Ǒ" => Ok(Letter(123)),
            "Ò" => Ok(Letter(124)),
            "U" => Ok(Letter(125)),
            "Ū" => Ok(Letter(126)),
            "Ú" => Ok(Letter(127)),
            "Ǔ" => Ok(Letter(128)),
            "Ù" => Ok(Letter(129)),
            "Ê" => Ok(Letter(130)),
            "Ê̄" => Ok(Letter(131)),
            "Ế" => Ok(Letter(132)),
            "Ê̌" => Ok(Letter(133)),
            "Ề" => Ok(Letter(134)),
            "Ü" => Ok(Letter(135)),
            "Ǖ" => Ok(Letter(136)),
            "Ǘ" => Ok(Letter(137)),
            "Ǚ" => Ok(Letter(138)),
            "Ǜ" => Ok(Letter(139)),
            "0" => Ok(Letter(140)),
            "1" => Ok(Letter(141)),
            "2" => Ok(Letter(142)),
            "3" => Ok(Letter(143)),
            "4" => Ok(Letter(144)),
            "5" => Ok(Letter(145)),
            "6" => Ok(Letter(146)),
            "7" => Ok(Letter(147)),
            "8" => Ok(Letter(148)),
            "9" => Ok(Letter(149)),
            "⁰" => Ok(Letter(150)),
            "¹" => Ok(Letter(151)),
            "²" => Ok(Letter(152)),
            "³" => Ok(Letter(153)),
            "⁴" => Ok(Letter(154)),
            "⁵" => Ok(Letter(155)),
            "⁶" => Ok(Letter(156)),
            "⁷" => Ok(Letter(157)),
            "⁸" => Ok(Letter(158)),
            "⁹" => Ok(Letter(159)),
            "˙" => Ok(Letter(160)),
            "ˉ" => Ok(Letter(161)),
            "ˊ" => Ok(Letter(162)),
            "ˇ" => Ok(Letter(163)),
            "ˋ" => Ok(Letter(164)),
            "'" => Ok(Letter(165)),
            _   => Err(()),
        }
    }
}


#[test]
fn test_letter() {

}