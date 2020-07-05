use std::io;
use std::char;

const CHARACTER_RANGE: u16 = 126 - 32;
const FIRST_CHARACTER: char = ' ';
const LAST_CHARACTER: char = '~';

fn main() {

    let mut shift: String = String::new();
    let mut input_text: String = String::new();

    println!("Please enter the text to be encrypted:");

    io::stdin().read_line(&mut input_text)
        .expect("Failed to read line!");

    println!("Please enter the desired shift:");

    io::stdin().read_line(&mut shift)
        .expect("Failed to read line!");

    let shift: i32 = match shift.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("enter a number!"); 0},
    };

    
    caesar_shift(shift, input_text);
}

fn caesar_shift(shift: i32, input_text: String) {
    let mut buf = [0; 2];
    for character in input_text.trim().chars() {
        println!("{}", character.encode_utf8(&mut buf));
        println!("  {}_{}", buf[0], buf[1]);
        let x: u32 = buf[0].into();
        println!("num {}", x);
        let test: char = match char::from_digit(x, 10) {Some(v) => v, None => 'a',};
        println!("   {}-", test);

        //println!("{}.", (character.to_digit(10) + shift));
    }
}
