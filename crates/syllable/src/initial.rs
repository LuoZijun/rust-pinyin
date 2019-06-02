use std::fmt;
use crate::tone::ToneFormat;


// 对于 《汉语拼音方案》 当中的声母表的补充说明
// `y` 和 `w` 在现代学说里面被称为 `零声母` ，
// 但是汉语拼音并不承认他的地位，所以它的出现与否应该按照 前缀补写规则 来。


// 注: ng 在普通话当中只作韵尾(韵母尾部)，在一些地方方言当中可以用作声母 (如: ňg).
//     我们这里并不支持。
/// 声母表
pub static INITIAL_TABLE: [&'static str; 21] = [
     "b",  "p",  "m", "f", "d", "t", "n", "l",
     "g",  "k",  "h",      "j", "q", "x", 
    "zh", "ch", "sh", "r", "z", "c", "s",
];

// 8 Bits
// 低 第1位 被用作指示韵母是否为简写形式
/// 声母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Initial(pub(crate) u8);


impl Initial {
    #[inline]
    pub fn offset(&self) -> u8 {
        self.0 >> 1
    }

    #[inline]
    pub fn is_simplified(&self) -> bool {
        self.0 & 0b0000_0001 == 1
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        let offset = self.offset();

        if self.is_simplified() {
            match offset {
                14 => "ẑ",
                15 => "ĉ",
                16 => "ŝ",
                _  => INITIAL_TABLE[offset as usize],
            }
        } else {
            INITIAL_TABLE[offset as usize]
        }
    }

    /// 转位简写形式 
    #[inline]
    pub fn simplified(&self) -> Self {
        // 声母的简写形式
        if self.is_simplified() {
            *self
        } else {
            Self(self.0 | 0b0000_0001)
        }
    }

    #[inline]
    pub fn format(&self, _fmt: ToneFormat) -> &'static str {
        self.as_str()
    }
}

impl fmt::Debug for Initial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}

impl fmt::Display for Initial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}