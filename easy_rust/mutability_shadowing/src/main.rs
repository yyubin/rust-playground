// mutability
// shadowing 같은 이름을 다시 쓰는 것

// immutalbe by default
// mut

fn double(input: i32) -> i32 {
    input * 2
}

fn triple(input: i32) -> i32 {
    input * 3
}

fn main() {
    // mutability
    let mut my_number = 10;
    my_number = 9;

    // shadowing
    let my_value = 10;
    let my_value = 11;
    let my_value = "My value";

    println!("{}", my_value); // My value

    let my_value = 10;
    let my_value = 11;

    // code block
    {
        let my_value = "My value";
        println!("{}", my_value); // My value
    }

    println!("{}", my_value); // 11

    let x = 9;
    let x = double(x);
    let x = triple(x);
    println!("{}", x);


}
