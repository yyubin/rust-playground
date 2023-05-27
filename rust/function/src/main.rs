fn main() {
    //another_function(5, 'h');
    //let x = (let y = 6); // 불가능

    let y = {
        let x = 3;
        x + 1
        // 표현식에는 종료 세미콜론이 포함되지 않습니다. 
        // 표현식 끝에 세미콜론을 추가하면 문장으로 바뀌고 값을 반환하지 않습니다.
    };

    println!("The value of y is: {y}"); //4

    let x = five();
    println!("The value of x is: {x}"); //5

    let x = plus_one(5);
    println!("The value of x is: {x}"); //6

}

fn plus_one(x: i32) -> i32 {
    x + 1
    // 세미콜론 추가할 시 컴파일 오류
}

fn five() -> i32 {
    5
}

// fn another_function() {
//     println!("Another function.")
// }

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}