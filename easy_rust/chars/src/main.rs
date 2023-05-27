use std::mem::size_of;

fn main() {
    // char
    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too

    // char : 4byte
    
    // casting : simple type change

    let my_number: u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number + second_number as u16;

    // ASCII 255개
    // u8 = 255

    let ascii_number = 'a' as u16;
    println!("{}", ascii_number);

    // len() : byte 글자수가 아닌 바이트 수
    println!("Size of a char: {}", size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len()); // 2
    println!("Size of string containing '国': {}", "国".len()); // 3
    println!("Size of string containing '𓅱': {}", "𓅱".len()); // 4

    let slice = "Hello!";
    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    let slice2 = "안녕!";
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}
