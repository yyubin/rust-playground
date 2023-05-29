// It's trivial to copy the bytes
// 소유권 신경쓸 필요없음
// Ownership and copy types
fn prints_number(number: i32) {
    println!("{number}");
}

fn prints_string(input: String) {
    println!("{input}")
}

// copy - copy types
// clone - non-copy types
fn main() {
    let my_number = 9;
    prints_number(my_number); // 9
    prints_number(my_number); // 9

    let my_string = "Austria".to_string();
    prints_string(my_string.clone()); // 클론을 전달함
                                      // 여기까지 진행 했을 경우 my_string 소유권은 아직 my_string 한테 있음
    prints_string(my_string);
}
