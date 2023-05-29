fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}

// uninitialized variable
// control flow


// possibly uninitialized = maybe doesn't have a value yet
fn main() {
    let my_number; // 가능
    
    {
        let x = loop_then_return(34);
        my_number = x
    };

    println!("{my_number}") // 위에 코드 블럭 넣으면 가능
}
