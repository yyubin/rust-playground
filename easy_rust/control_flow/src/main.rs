fn match_colors(rbg: (u32, u32, u32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not match Red"),
        (_, b, _) if b < 10 => println!("Not match Blue"),
        (_, _, g) if g < 10 => println!("Not match Green"),
        _ => println!("Every color has at least 10")
    }
}

fn match_number(input: i32) {
    match input {
        // number => println!("It's the number {number}"),
        number @ 0..=10 => println!("between 0 and 10. It's {number}"),
        _ => println!("else")
    }
}

fn main() {
    let my_num = 5;
    let my_sec_num = 10;
    if my_num == 5 && my_sec_num == 10 {
        println!("They both match");
    } else if my_num == 6 { // && || 
        println!("It's six");
    } else {
        println!("It's different");
    }

    // match
    let my_num: u8 = 5;

    match my_num { // switch
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("different") // _ "I dont care" "anything"
    }

    let sec_num = match my_num {
        0 => 23,
        1 => 65,
        _ => 0
    };

    println!("sec_num is : {}", sec_num);

    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("nice today"),
        ("clear", "warm") => println!("nice day"),
        (_, "warm") => println!("warm!!"),
        _ => println!("not sure what the weather is.")
    }

    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if !married => println!("Not married with {children}"),
        (c, m) if c == 0 && married => println!("Married but with no children"),
        _ => println!("other type")
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colors(first);
    match_colors(second);
    match_colors(third);

    let my_num = 10;
    let some_val = match my_num {
        10 => 8,
        // _ => "Not ten" // 불가능
        _ => 7
    };

    let some_val = if my_num == 10 {8} else {7};

    match_number(10);

}
