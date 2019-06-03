#[allow(unused_imports)]

use syllable::pinyin::{ PinYin, PINYIN_TABLE, };
use syllable::tone::{ Tone, ToneFormat, };


fn main() {
    let s = PinYin::new_unchecked(21 << 1);
    println!("{:?}", s);
    println!("{:?}", s.consonant());
    
    println!("{:?}", s.format(Tone::Second, ToneFormat::Plain));
    println!("{:?}", s.format(Tone::Second, ToneFormat::Mark));
    println!("{:?}", s.format(Tone::Second, ToneFormat::Number));
    
    // let i = s.0 as usize;
    
    // // println!("{} {:?}", s, &PINYIN_TABLE[i + 10 ..  i+100]);
    // println!("{:?}", PINYIN_TABLE[i],);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 1]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 2]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 3]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 4]);

    // println!("{:?}", PINYIN_TABLE[i + 924 * 4 + 924 * 1]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 4 + 924 * 2]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 4 + 924 * 3]);
    // println!("{:?}", PINYIN_TABLE[i + 924 * 4 + 924 * 4]);

}