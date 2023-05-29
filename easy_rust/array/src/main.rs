// Collection types
// Array


// &str
fn main() {
    let array = ["One", "Two"];
    let array2 = ["One", "Two", "Five"];
    // array와 array2는 서로 다른 타입

    // 크기에 따라 완전 다른타입이 되어서 하단은 실행 불가
    //println!("Is array the same as array2? {}", array == array2);
    // [&str; 2] / [&str; 3]

    let array = ["One", "Two"];
    let array2 = ["One", "Two"];
    println!("Is array the same as array2? {}", array == array2); // true

    // 추가 삭제 비교 불가, 기본적이고 아주 빠른 타입

    let array = ["One", "Two"];

    // array.asdfasdf(); // 컴파일 불가하지만 타입을 알수 있음
    // no method named `asdfasdf` found for array `[&str; 2]` in the current scope

    // array는 buffer 만들때 사용
    let array = [0; 64]; // 0을 64번
    println!("{:?}", array);

    // array는 변하지 않는 것들을 넣음

    let array = ["1월", "2월"]; // indexing
    println!("{}", array[1]);
    // println!("{}", array[2]); // 불가능
    println!("{:?}", array.get(3)); // None


    ////////
    // Slice
    ////////

    // dynamically sized type
    // &str
    let seasons = ["Spring", "Summer", "Autumn", "Winter"];
    println!("{:?}", &seasons[0..2]); // ["Spring", "Summer"]
    println!("{:?}", &seasons[0..=2]); // ["Spring", "Summer", "Autumn"]
    println!("{:?}", &seasons[..]); // ["Spring", "Summer", "Autumn", "Winter"]
    println!("{:?}", &seasons[..=3]); // up to and including ["Spring", "Summer", "Autumn", "Winter"]



    //// Vec
    // String도 Vec으로 만듦
}
