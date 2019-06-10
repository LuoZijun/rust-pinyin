#![allow(unused_imports)]

use syllable::pinyin::{ PinYin, PINYIN_TABLE, };
use syllable::tone::{ Tone, ToneFormat, };


fn main() {

    let pinyin = PinYin::new_unchecked(21 << 1);
    println!("{:?}", pinyin);
    println!("initials: {:?}", pinyin.initials());
    println!("consonant: {:?}", pinyin.consonant());
    println!("finals: {:?}", pinyin.finals());
    println!("rhyme: {:?}", pinyin.rhyme());

    // println!("{:?}", s.format(Tone::Second, ToneFormat::Plain));
    // println!("{:?}", s.format(Tone::Second, ToneFormat::Mark));
    // println!("{:?}", s.format(Tone::Second, ToneFormat::Number));
    
    let pinyin = PinYin::new_unchecked((20 * 22) << 1);
    println!("{:?}", pinyin);
    println!("initials: {:?}", pinyin.initials());
    println!("consonant: {:?}", pinyin.consonant());
    println!("finals: {:?}", pinyin.finals());
    println!("rhyme: {:?}", pinyin.rhyme());
    
    let pinyin = PinYin::new_unchecked((28 * 22) << 1);
    println!("{:?}", pinyin);
    println!("initials: {:?}", pinyin.initials());
    println!("consonant: {:?}", pinyin.consonant());
    println!("finals: {:?}", pinyin.finals());
    println!("rhyme: {:?}", pinyin.rhyme());
    
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