use core::fmt;

use crate::tone::ToneFormat;


// 对于 《汉语拼音方案》 当中的声母表的补充说明
// `y` 和 `w` 在现代学说里面被称为 `零声母` ，
// 但是汉语拼音并不承认他的地位，所以它的出现与否应该按照 前缀补写规则 来。

// 注: ng 在普通话当中只作韵尾(韵母尾部)，在一些地方方言当中可以用作声母 (如: ňg).
//     我们这里并不支持。
/// 声母表
#[rustfmt::skip]
pub static INITIAL_TABLE: [&'static str; 21] = [
     "b",  "p",  "m", "f", "d", "t", "n", "l",
     "g",  "k",  "h",      "j", "q", "x", 
    "zh", "ch", "sh", "r", "z", "c", "s",
    // "ẑ", "ĉ", "ŝ",
];

// 8 Bits
// 低 第1位 被用作指示韵母是否为简写形式
/// 声母
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Initial(pub(crate) u8);


impl Initial {
    pub const B: Self = Initial(0);
    pub const P: Self = Initial(1 << 1);
    pub const M: Self = Initial(2 << 1);
    pub const F: Self = Initial(3 << 1);
    pub const D: Self = Initial(4 << 1);

    pub const T: Self = Initial(5 << 1);
    pub const N: Self = Initial(6 << 1);
    pub const L: Self = Initial(7 << 1);
    pub const G: Self = Initial(8 << 1);
    pub const K: Self = Initial(9 << 1);
    pub const H: Self = Initial(10 << 1);

    pub const J: Self = Initial(11 << 1);
    pub const Q: Self = Initial(12 << 1);
    pub const X: Self = Initial(13 << 1);

    pub const ZH: Self = Initial(14 << 1);
    pub const CH: Self = Initial(15 << 1);
    pub const SH: Self = Initial(16 << 1);

    pub const R: Self = Initial(17 << 1);
    pub const Z: Self = Initial(18 << 1);
    pub const C: Self = Initial(19 << 1);
    pub const S: Self = Initial(20 << 1);

    #[inline]
    pub fn offset(&self) -> u8 {
        self.0 >> 1
    }

    #[inline]
    pub fn is_simplified(&self) -> bool {
        self.0 & 0b0000_0001 == 1
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


#[test]
fn test_initial_const() {
    assert_eq!(Initial::B.as_str(), "b");
    assert_eq!(Initial::P.as_str(), "p");
    assert_eq!(Initial::M.as_str(), "m");
    assert_eq!(Initial::F.as_str(), "f");
    assert_eq!(Initial::D.as_str(), "d");
}