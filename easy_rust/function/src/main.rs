fn give_number(one: i32, two: i32) -> i32 {
    one * two
}

fn print_number(one: i32, two: i32) -> i32 {
    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    multiplied_by_ten
}

fn main() {
    let my_number = give_number(9, 8);
    println!("{}", my_number);
    println!("{}", print_number(9, 8));
}
