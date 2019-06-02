use std::fmt;


// Unicode 修饰字符
// 第一声: \u0304  ̄
// 第二声: \u0301  ́
// 第三声: \u030C  ̌
// 第四声: \u0300  ̀

// 声调: ˉ ˊ ˇ ˋ ˙


/// 音调标记方式
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ToneFormat {
    /// 不标示音调
    Ignore,
    /// 带音调符号的拼音字母 ( fan, fān )
    Symbol,
    /// 数字法 ( fan, fɑn⁵⁵, fan³⁵ )
    Digit,
    /// 声序法 ( fan, fan1 )
    Index,
}


/// 声调
#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Tone {
    /// 第零声: 轻声调(不标符号)
    Neutral = 0u8,
    /// 第一声: 平调(阴平)
    First,
    /// 第二声: 升调(阳平)
    Second,
    /// 第三声: 降升语调(上升或折调)
    Third,
    /// 第四声: 降调(去声)
    Fourth,
}

impl Into<u8> for Tone {
    fn into(self) -> u8 {
        use self::Tone::*;

        match self {
            Neutral => 0u8,
            First => 1,
            Second => 2,
            Third => 3,
            Fourth => 4,
        }
    }
}

impl Tone {
    // 调值
    #[inline]
    pub fn value(&self) -> Option<u8> {
        use self::Tone::*;

        match *self {
            Neutral => None,
            First => Some(55u8),
            Second => Some(35u8),
            Third => Some(214u8),
            Fourth => Some(51u8),
        }
    }

    // TODO: 输出 Letter 格式
    #[inline]
    pub fn mark(&self) -> &'static str {
        use self::Tone::*;
        // ˉ ˊ ˇ ˋ ˙
        match *self {
            Neutral => "˙",
            First => "ˉ",
            Second => "ˊ",
            Third => "ˇ",
            Fourth => "ˋ",
        }
    }
}

impl fmt::Debug for Tone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.mark())
    }
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mark())
    }
}
