use std::io;

fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // let spaces = "   ";
    // let spaces = spaces.len(); // 오류 발생 X
    // let mut spaces = "   ";
    // spaces = spaces.len(); // 오류 발생 O

    let _guess: u32 = "42".parse().expect("Not a number!");
    // u32 : 부호 없는 32bit 정수형
    // i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    // 정수 기본값 : i32

    // 정수 오버 플로
    // 오버플로가 발생하면 Rust는 2의 보수 래핑 을 수행합니다. 
    // 요컨대, 유형이 보유할 수 있는 최대값보다 큰 값은 유형이 보유할 수 있는 최소값으로 "랩 어라운드"됩니다. 
    // a 의 경우 u8값 256은 0이 되고 값 257은 1이 됩니다.

    // 오버 플로 처리 표준 라이브러리
    // Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // Return the None value if there is overflow with the checked_* methods.
    // Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
    // Saturate at the value’s minimum or maximum values with the saturating_* methods.

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
    
    // multiplication
    let product = 4 * 30;
    
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    
    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;
    let f: bool = false;

    // char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 데이터를 스택에 할당하거나, 고정된 수의 요소를 갖출때 사용
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first : {first}");
    println!("second : {second}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
