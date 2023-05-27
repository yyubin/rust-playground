// macro : 코드를 만드는 함수, function that writes code
fn give_age() -> i32 {
    10
}

fn main() {
    println!("Hello, world!");

    let my_name = "yubin";
    let my_age = 10;
    println!("My name is {} and my age is {}", my_name, give_age());
    println!("My name is {my_name} and my age is {my_age}");

    let my_city = "Seoul";
    let year = 2000;
    let population = 9_999_999;
    // string interpolation
    println!("The city of {my_city} in {year} had a population of {population}");
    
    println!(
        "The city fo {0} in {1} had a population of {2}",
        my_city,
        year,
        population
    );

    // expression
    println!("{{1+5}}"); // {1+5}


    /////////
    // fancy print
    /////////

    print!("This\nis\na\nbunch\nof\nlines");
    print!("This\nis\na\nbunch\nof\nlines");
    print!(r#"c:\thisdrive\new_drive"#); // raw text
    print!("Let me tell you
        어떤
        가자
    \n");

    let my_value = &9;
    println!("{:p}", my_value);
    let my_value = 9000;
    println!("{:b}", my_value); // byte
    println!("{:X}", my_value); // hex decimal

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in center, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right

}   
