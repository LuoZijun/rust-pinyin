

pub mod tone;
pub mod letter;
pub mod initial;
pub mod rhyme;
pub mod pinyin;


pub(crate) const SP: &'static str = " ";



// LetterIter
pub struct Letters {

}

impl Iterator for Letters {
    type Item = letter::Letter;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}


pub trait Syllable {
    // 拼音表偏移量
    fn offset(&self) -> u16;

    // 音调
    fn tone(&self) -> tone::Tone;
    // 拼音音调标记形式
    fn tone_format(&self) -> tone::ToneFormat;

    // 元音字母
    fn vowel(&self) -> letter::Letter;
    // 携带音调的元音字母
    fn tone_mark(&self) -> letter::Letter;

    // 松散数据
    // 声母部分
    fn initials(&self) -> Option<&'static str>; // 支持输出 Y/W
    // 韵母部分，改写或补写后的形式
    // 不携带声调 以及简写形式字母
    fn finals(&self) -> &'static str;

    // 结构化数据
    // 声母
    fn consonant(&self) -> Option<initial::Initial>; // 不支持 Y/W
    // 韵母 (原始形式)
    fn rhyme(&self) -> rhyme::Rhyme;
    
    // 儿化音结尾 ？
    fn is_er(&self) -> bool;

    // 是否为简写形式
    fn is_simplified(&self) -> bool;
    
    // 拼音字母列表
    fn letters(&self) -> Letters; // 类似于 String::chars() -> Chars

    // 不带声调以及不使用简写字母的形式
    fn plain(&self) -> Self;
    // 转换为简写形式
    fn simplified(&self) -> Self;
    
    // 输出
    fn as_str(&self) -> &'static str;

    // 重设音调
    fn with_tone(&self, tone: tone::Tone) -> Self;
    // 重设音调标记形式
    fn with_tone_fotmat(&self, tone_format: tone::ToneFormat) -> Self;
    // 重设音调以及音调标记形式
    fn formart(&self, tone: tone::Tone, tone_format: tone::ToneFormat) -> Self;
}
