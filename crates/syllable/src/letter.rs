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
    "0", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
    "⁰", "¹", "²", "³", "⁴", 
    "⁵", "⁶", "⁷", "⁸", "⁹",
    // 音调符号
    "˙", "ˉ", "ˊ", "ˇ", "ˋ",
    // 隔音符号
    "'",

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
];


// 8 Bits
// 拼音字母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Letter(pub(crate) u8);

impl Letter {
    pub const MIN: Self = Letter(0);
    pub const MAX: Self = Letter(140);

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

    pub fn len_gbk(self) -> usize {
        unimplemented!()
    }

    pub fn len_gb2312(self) -> usize {
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

    pub fn encode_gbk(self, _dst: &mut [u8]) -> usize {
        unimplemented!()
    }

    pub fn encode_gb2312(self, _dst: &mut [u8]) -> usize {
        unimplemented!()
    }

    pub fn encode_gb18030(self, _dst: &mut [u8]) -> usize {
        unimplemented!()
    }

    pub fn is_lowercase(self) -> bool {
        unimplemented!()
    }

    pub fn is_uppercase(self) -> bool {
        unimplemented!()
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

    

    pub fn is_initial(self) -> bool {
        unimplemented!()
    }
    pub fn is_final(self) -> bool {
        unimplemented!()
    }

    pub fn is_vowel(self) -> bool {
        unimplemented!()
    }
    pub fn is_rhyme(self) -> bool {
        unimplemented!()
    }
}